#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = i8;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = i32;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int32_t, __int8_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "633:16"]
    pub struct x264_level_t {
        pub level_idc: uint8_t,
        pub mbps: int32_t,
        pub frame_size: int32_t,
        pub dpb: int32_t,
        pub bitrate: int32_t,
        pub cpb: int32_t,
        pub mv_range: uint16_t,
        pub mvs_per_2mb: uint8_t,
        pub slice_rate: uint8_t,
        pub mincr: uint8_t,
        pub bipred8x8: uint8_t,
        pub direct8x8: uint8_t,
        pub frame_only: uint8_t,
    }
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint16_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:0"]
pub mod tables_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct vlc_t {
        pub i_bits: uint8_t,
        pub i_size: uint8_t,
    }
    use super::stdint_uintn_h::uint8_t;
}
pub use self::stdint_intn_h::{int32_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
pub use self::tables_h::vlc_t;
pub use self::types_h::{__int32_t, __int8_t, __uint16_t, __uint32_t, __uint8_t};
pub use self::x264_h::x264_level_t;
#[no_mangle]
#[c2rust::src_loc = "29:20"]
pub static mut x264_levels: [x264_level_t; 21] = [
    {
        let mut init = x264_level_t {
            level_idc: 10 as uint8_t,
            mbps: 1485 as int32_t,
            frame_size: 99 as int32_t,
            dpb: 396 as int32_t,
            bitrate: 64 as int32_t,
            cpb: 175 as int32_t,
            mv_range: 64 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 9 as uint8_t,
            mbps: 1485 as int32_t,
            frame_size: 99 as int32_t,
            dpb: 396 as int32_t,
            bitrate: 128 as int32_t,
            cpb: 350 as int32_t,
            mv_range: 64 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 11 as uint8_t,
            mbps: 3000 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 900 as int32_t,
            bitrate: 192 as int32_t,
            cpb: 500 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 12 as uint8_t,
            mbps: 6000 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 384 as int32_t,
            cpb: 1000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 13 as uint8_t,
            mbps: 11880 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 768 as int32_t,
            cpb: 2000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 20 as uint8_t,
            mbps: 11880 as int32_t,
            frame_size: 396 as int32_t,
            dpb: 2376 as int32_t,
            bitrate: 2000 as int32_t,
            cpb: 2000 as int32_t,
            mv_range: 128 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 21 as uint8_t,
            mbps: 19800 as int32_t,
            frame_size: 792 as int32_t,
            dpb: 4752 as int32_t,
            bitrate: 4000 as int32_t,
            cpb: 4000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 22 as uint8_t,
            mbps: 20250 as int32_t,
            frame_size: 1620 as int32_t,
            dpb: 8100 as int32_t,
            bitrate: 4000 as int32_t,
            cpb: 4000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 64 as uint8_t,
            slice_rate: 0 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 0 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 30 as uint8_t,
            mbps: 40500 as int32_t,
            frame_size: 1620 as int32_t,
            dpb: 8100 as int32_t,
            bitrate: 10000 as int32_t,
            cpb: 10000 as int32_t,
            mv_range: 256 as uint16_t,
            mvs_per_2mb: 32 as uint8_t,
            slice_rate: 22 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 0 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 31 as uint8_t,
            mbps: 108000 as int32_t,
            frame_size: 3600 as int32_t,
            dpb: 18000 as int32_t,
            bitrate: 14000 as int32_t,
            cpb: 14000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 60 as uint8_t,
            mincr: 4 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 32 as uint8_t,
            mbps: 216000 as int32_t,
            frame_size: 5120 as int32_t,
            dpb: 20480 as int32_t,
            bitrate: 20000 as int32_t,
            cpb: 20000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 60 as uint8_t,
            mincr: 4 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 40 as uint8_t,
            mbps: 245760 as int32_t,
            frame_size: 8192 as int32_t,
            dpb: 32768 as int32_t,
            bitrate: 20000 as int32_t,
            cpb: 25000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 60 as uint8_t,
            mincr: 4 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 41 as uint8_t,
            mbps: 245760 as int32_t,
            frame_size: 8192 as int32_t,
            dpb: 32768 as int32_t,
            bitrate: 50000 as int32_t,
            cpb: 62500 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 0 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 42 as uint8_t,
            mbps: 522240 as int32_t,
            frame_size: 8704 as int32_t,
            dpb: 34816 as int32_t,
            bitrate: 50000 as int32_t,
            cpb: 62500 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 50 as uint8_t,
            mbps: 589824 as int32_t,
            frame_size: 22080 as int32_t,
            dpb: 110400 as int32_t,
            bitrate: 135000 as int32_t,
            cpb: 135000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 51 as uint8_t,
            mbps: 983040 as int32_t,
            frame_size: 36864 as int32_t,
            dpb: 184320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 52 as uint8_t,
            mbps: 2073600 as int32_t,
            frame_size: 36864 as int32_t,
            dpb: 184320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 512 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 60 as uint8_t,
            mbps: 4177920 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 240000 as int32_t,
            cpb: 240000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 61 as uint8_t,
            mbps: 8355840 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 480000 as int32_t,
            cpb: 480000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 62 as uint8_t,
            mbps: 16711680 as int32_t,
            frame_size: 139264 as int32_t,
            dpb: 696320 as int32_t,
            bitrate: 800000 as int32_t,
            cpb: 800000 as int32_t,
            mv_range: 8192 as uint16_t,
            mvs_per_2mb: 16 as uint8_t,
            slice_rate: 24 as uint8_t,
            mincr: 2 as uint8_t,
            bipred8x8: 1 as uint8_t,
            direct8x8: 1 as uint8_t,
            frame_only: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = x264_level_t {
            level_idc: 0 as uint8_t,
            mbps: 0,
            frame_size: 0,
            dpb: 0,
            bitrate: 0,
            cpb: 0,
            mv_range: 0,
            mvs_per_2mb: 0,
            slice_rate: 0,
            mincr: 0,
            bipred8x8: 0,
            direct8x8: 0,
            frame_only: 0,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "58:15"]
pub static mut x264_exp2_lut: [uint8_t; 64] = [
    0 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    45 as ::core::ffi::c_int as uint8_t,
    48 as ::core::ffi::c_int as uint8_t,
    52 as ::core::ffi::c_int as uint8_t,
    55 as ::core::ffi::c_int as uint8_t,
    58 as ::core::ffi::c_int as uint8_t,
    62 as ::core::ffi::c_int as uint8_t,
    65 as ::core::ffi::c_int as uint8_t,
    69 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    80 as ::core::ffi::c_int as uint8_t,
    83 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    91 as ::core::ffi::c_int as uint8_t,
    94 as ::core::ffi::c_int as uint8_t,
    98 as ::core::ffi::c_int as uint8_t,
    102 as ::core::ffi::c_int as uint8_t,
    106 as ::core::ffi::c_int as uint8_t,
    110 as ::core::ffi::c_int as uint8_t,
    114 as ::core::ffi::c_int as uint8_t,
    118 as ::core::ffi::c_int as uint8_t,
    122 as ::core::ffi::c_int as uint8_t,
    126 as ::core::ffi::c_int as uint8_t,
    130 as ::core::ffi::c_int as uint8_t,
    135 as ::core::ffi::c_int as uint8_t,
    139 as ::core::ffi::c_int as uint8_t,
    143 as ::core::ffi::c_int as uint8_t,
    147 as ::core::ffi::c_int as uint8_t,
    152 as ::core::ffi::c_int as uint8_t,
    156 as ::core::ffi::c_int as uint8_t,
    161 as ::core::ffi::c_int as uint8_t,
    165 as ::core::ffi::c_int as uint8_t,
    170 as ::core::ffi::c_int as uint8_t,
    175 as ::core::ffi::c_int as uint8_t,
    179 as ::core::ffi::c_int as uint8_t,
    184 as ::core::ffi::c_int as uint8_t,
    189 as ::core::ffi::c_int as uint8_t,
    194 as ::core::ffi::c_int as uint8_t,
    198 as ::core::ffi::c_int as uint8_t,
    203 as ::core::ffi::c_int as uint8_t,
    208 as ::core::ffi::c_int as uint8_t,
    214 as ::core::ffi::c_int as uint8_t,
    219 as ::core::ffi::c_int as uint8_t,
    224 as ::core::ffi::c_int as uint8_t,
    229 as ::core::ffi::c_int as uint8_t,
    234 as ::core::ffi::c_int as uint8_t,
    240 as ::core::ffi::c_int as uint8_t,
    245 as ::core::ffi::c_int as uint8_t,
    250 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "66:13"]
pub static mut x264_log2_lut: [::core::ffi::c_float; 128] = [
    0.00000f64 as ::core::ffi::c_float,
    0.01123f64 as ::core::ffi::c_float,
    0.02237f64 as ::core::ffi::c_float,
    0.03342f64 as ::core::ffi::c_float,
    0.04439f64 as ::core::ffi::c_float,
    0.05528f64 as ::core::ffi::c_float,
    0.06609f64 as ::core::ffi::c_float,
    0.07682f64 as ::core::ffi::c_float,
    0.08746f64 as ::core::ffi::c_float,
    0.09803f64 as ::core::ffi::c_float,
    0.10852f64 as ::core::ffi::c_float,
    0.11894f64 as ::core::ffi::c_float,
    0.12928f64 as ::core::ffi::c_float,
    0.13955f64 as ::core::ffi::c_float,
    0.14975f64 as ::core::ffi::c_float,
    0.15987f64 as ::core::ffi::c_float,
    0.16993f64 as ::core::ffi::c_float,
    0.17991f64 as ::core::ffi::c_float,
    0.18982f64 as ::core::ffi::c_float,
    0.19967f64 as ::core::ffi::c_float,
    0.20945f64 as ::core::ffi::c_float,
    0.21917f64 as ::core::ffi::c_float,
    0.22882f64 as ::core::ffi::c_float,
    0.23840f64 as ::core::ffi::c_float,
    0.24793f64 as ::core::ffi::c_float,
    0.25739f64 as ::core::ffi::c_float,
    0.26679f64 as ::core::ffi::c_float,
    0.27612f64 as ::core::ffi::c_float,
    0.28540f64 as ::core::ffi::c_float,
    0.29462f64 as ::core::ffi::c_float,
    0.30378f64 as ::core::ffi::c_float,
    0.31288f64 as ::core::ffi::c_float,
    0.32193f64 as ::core::ffi::c_float,
    0.33092f64 as ::core::ffi::c_float,
    0.33985f64 as ::core::ffi::c_float,
    0.34873f64 as ::core::ffi::c_float,
    0.35755f64 as ::core::ffi::c_float,
    0.36632f64 as ::core::ffi::c_float,
    0.37504f64 as ::core::ffi::c_float,
    0.38370f64 as ::core::ffi::c_float,
    0.39232f64 as ::core::ffi::c_float,
    0.40088f64 as ::core::ffi::c_float,
    0.40939f64 as ::core::ffi::c_float,
    0.41785f64 as ::core::ffi::c_float,
    0.42626f64 as ::core::ffi::c_float,
    0.43463f64 as ::core::ffi::c_float,
    0.44294f64 as ::core::ffi::c_float,
    0.45121f64 as ::core::ffi::c_float,
    0.45943f64 as ::core::ffi::c_float,
    0.46761f64 as ::core::ffi::c_float,
    0.47573f64 as ::core::ffi::c_float,
    0.48382f64 as ::core::ffi::c_float,
    0.49185f64 as ::core::ffi::c_float,
    0.49985f64 as ::core::ffi::c_float,
    0.50779f64 as ::core::ffi::c_float,
    0.51570f64 as ::core::ffi::c_float,
    0.52356f64 as ::core::ffi::c_float,
    0.53138f64 as ::core::ffi::c_float,
    0.53916f64 as ::core::ffi::c_float,
    0.54689f64 as ::core::ffi::c_float,
    0.55459f64 as ::core::ffi::c_float,
    0.56224f64 as ::core::ffi::c_float,
    0.56986f64 as ::core::ffi::c_float,
    0.57743f64 as ::core::ffi::c_float,
    0.58496f64 as ::core::ffi::c_float,
    0.59246f64 as ::core::ffi::c_float,
    0.59991f64 as ::core::ffi::c_float,
    0.60733f64 as ::core::ffi::c_float,
    0.61471f64 as ::core::ffi::c_float,
    0.62205f64 as ::core::ffi::c_float,
    0.62936f64 as ::core::ffi::c_float,
    0.63662f64 as ::core::ffi::c_float,
    0.64386f64 as ::core::ffi::c_float,
    0.65105f64 as ::core::ffi::c_float,
    0.65821f64 as ::core::ffi::c_float,
    0.66534f64 as ::core::ffi::c_float,
    0.67243f64 as ::core::ffi::c_float,
    0.67948f64 as ::core::ffi::c_float,
    0.68650f64 as ::core::ffi::c_float,
    0.69349f64 as ::core::ffi::c_float,
    0.70044f64 as ::core::ffi::c_float,
    0.70736f64 as ::core::ffi::c_float,
    0.71425f64 as ::core::ffi::c_float,
    0.72110f64 as ::core::ffi::c_float,
    0.72792f64 as ::core::ffi::c_float,
    0.73471f64 as ::core::ffi::c_float,
    0.74147f64 as ::core::ffi::c_float,
    0.74819f64 as ::core::ffi::c_float,
    0.75489f64 as ::core::ffi::c_float,
    0.76155f64 as ::core::ffi::c_float,
    0.76818f64 as ::core::ffi::c_float,
    0.77479f64 as ::core::ffi::c_float,
    0.78136f64 as ::core::ffi::c_float,
    0.78790f64 as ::core::ffi::c_float,
    0.79442f64 as ::core::ffi::c_float,
    0.80090f64 as ::core::ffi::c_float,
    0.80735f64 as ::core::ffi::c_float,
    0.81378f64 as ::core::ffi::c_float,
    0.82018f64 as ::core::ffi::c_float,
    0.82655f64 as ::core::ffi::c_float,
    0.83289f64 as ::core::ffi::c_float,
    0.83920f64 as ::core::ffi::c_float,
    0.84549f64 as ::core::ffi::c_float,
    0.85175f64 as ::core::ffi::c_float,
    0.85798f64 as ::core::ffi::c_float,
    0.86419f64 as ::core::ffi::c_float,
    0.87036f64 as ::core::ffi::c_float,
    0.87652f64 as ::core::ffi::c_float,
    0.88264f64 as ::core::ffi::c_float,
    0.88874f64 as ::core::ffi::c_float,
    0.89482f64 as ::core::ffi::c_float,
    0.90087f64 as ::core::ffi::c_float,
    0.90689f64 as ::core::ffi::c_float,
    0.91289f64 as ::core::ffi::c_float,
    0.91886f64 as ::core::ffi::c_float,
    0.92481f64 as ::core::ffi::c_float,
    0.93074f64 as ::core::ffi::c_float,
    0.93664f64 as ::core::ffi::c_float,
    0.94251f64 as ::core::ffi::c_float,
    0.94837f64 as ::core::ffi::c_float,
    0.95420f64 as ::core::ffi::c_float,
    0.96000f64 as ::core::ffi::c_float,
    0.96578f64 as ::core::ffi::c_float,
    0.97154f64 as ::core::ffi::c_float,
    0.97728f64 as ::core::ffi::c_float,
    0.98299f64 as ::core::ffi::c_float,
    0.98868f64 as ::core::ffi::c_float,
    0.99435f64 as ::core::ffi::c_float,
];
#[no_mangle]
#[c2rust::src_loc = "87:13"]
pub static mut x264_log2_lz_lut: [::core::ffi::c_float; 32] = [
    31 as ::core::ffi::c_int as ::core::ffi::c_float,
    30 as ::core::ffi::c_int as ::core::ffi::c_float,
    29 as ::core::ffi::c_int as ::core::ffi::c_float,
    28 as ::core::ffi::c_int as ::core::ffi::c_float,
    27 as ::core::ffi::c_int as ::core::ffi::c_float,
    26 as ::core::ffi::c_int as ::core::ffi::c_float,
    25 as ::core::ffi::c_int as ::core::ffi::c_float,
    24 as ::core::ffi::c_int as ::core::ffi::c_float,
    23 as ::core::ffi::c_int as ::core::ffi::c_float,
    22 as ::core::ffi::c_int as ::core::ffi::c_float,
    21 as ::core::ffi::c_int as ::core::ffi::c_float,
    20 as ::core::ffi::c_int as ::core::ffi::c_float,
    19 as ::core::ffi::c_int as ::core::ffi::c_float,
    18 as ::core::ffi::c_int as ::core::ffi::c_float,
    17 as ::core::ffi::c_int as ::core::ffi::c_float,
    16 as ::core::ffi::c_int as ::core::ffi::c_float,
    15 as ::core::ffi::c_int as ::core::ffi::c_float,
    14 as ::core::ffi::c_int as ::core::ffi::c_float,
    13 as ::core::ffi::c_int as ::core::ffi::c_float,
    12 as ::core::ffi::c_int as ::core::ffi::c_float,
    11 as ::core::ffi::c_int as ::core::ffi::c_float,
    10 as ::core::ffi::c_int as ::core::ffi::c_float,
    9 as ::core::ffi::c_int as ::core::ffi::c_float,
    8 as ::core::ffi::c_int as ::core::ffi::c_float,
    7 as ::core::ffi::c_int as ::core::ffi::c_float,
    6 as ::core::ffi::c_int as ::core::ffi::c_float,
    5 as ::core::ffi::c_int as ::core::ffi::c_float,
    4 as ::core::ffi::c_int as ::core::ffi::c_float,
    3 as ::core::ffi::c_int as ::core::ffi::c_float,
    2 as ::core::ffi::c_int as ::core::ffi::c_float,
    1 as ::core::ffi::c_int as ::core::ffi::c_float,
    0 as ::core::ffi::c_int as ::core::ffi::c_float,
];
#[no_mangle]
#[c2rust::src_loc = "97:16"]
pub static mut x264_lambda_tab: [uint16_t; 82] = [
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    1 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    2 as ::core::ffi::c_int as uint16_t,
    3 as ::core::ffi::c_int as uint16_t,
    3 as ::core::ffi::c_int as uint16_t,
    3 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    4 as ::core::ffi::c_int as uint16_t,
    5 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
    6 as ::core::ffi::c_int as uint16_t,
    7 as ::core::ffi::c_int as uint16_t,
    8 as ::core::ffi::c_int as uint16_t,
    9 as ::core::ffi::c_int as uint16_t,
    10 as ::core::ffi::c_int as uint16_t,
    11 as ::core::ffi::c_int as uint16_t,
    13 as ::core::ffi::c_int as uint16_t,
    14 as ::core::ffi::c_int as uint16_t,
    16 as ::core::ffi::c_int as uint16_t,
    18 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    23 as ::core::ffi::c_int as uint16_t,
    25 as ::core::ffi::c_int as uint16_t,
    29 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    36 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    45 as ::core::ffi::c_int as uint16_t,
    51 as ::core::ffi::c_int as uint16_t,
    57 as ::core::ffi::c_int as uint16_t,
    64 as ::core::ffi::c_int as uint16_t,
    72 as ::core::ffi::c_int as uint16_t,
    81 as ::core::ffi::c_int as uint16_t,
    91 as ::core::ffi::c_int as uint16_t,
    102 as ::core::ffi::c_int as uint16_t,
    114 as ::core::ffi::c_int as uint16_t,
    128 as ::core::ffi::c_int as uint16_t,
    144 as ::core::ffi::c_int as uint16_t,
    161 as ::core::ffi::c_int as uint16_t,
    181 as ::core::ffi::c_int as uint16_t,
    203 as ::core::ffi::c_int as uint16_t,
    228 as ::core::ffi::c_int as uint16_t,
    256 as ::core::ffi::c_int as uint16_t,
    287 as ::core::ffi::c_int as uint16_t,
    323 as ::core::ffi::c_int as uint16_t,
    362 as ::core::ffi::c_int as uint16_t,
    406 as ::core::ffi::c_int as uint16_t,
    456 as ::core::ffi::c_int as uint16_t,
    512 as ::core::ffi::c_int as uint16_t,
    575 as ::core::ffi::c_int as uint16_t,
    645 as ::core::ffi::c_int as uint16_t,
    724 as ::core::ffi::c_int as uint16_t,
    813 as ::core::ffi::c_int as uint16_t,
    912 as ::core::ffi::c_int as uint16_t,
    1024 as ::core::ffi::c_int as uint16_t,
    1149 as ::core::ffi::c_int as uint16_t,
    1290 as ::core::ffi::c_int as uint16_t,
    1448 as ::core::ffi::c_int as uint16_t,
    1625 as ::core::ffi::c_int as uint16_t,
    1825 as ::core::ffi::c_int as uint16_t,
    2048 as ::core::ffi::c_int as uint16_t,
    2299 as ::core::ffi::c_int as uint16_t,
    2580 as ::core::ffi::c_int as uint16_t,
    2896 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "114:11"]
pub static mut x264_lambda2_tab: [::core::ffi::c_int; 82] = [
    14 as ::core::ffi::c_int,
    18 as ::core::ffi::c_int,
    22 as ::core::ffi::c_int,
    28 as ::core::ffi::c_int,
    36 as ::core::ffi::c_int,
    45 as ::core::ffi::c_int,
    57 as ::core::ffi::c_int,
    72 as ::core::ffi::c_int,
    91 as ::core::ffi::c_int,
    115 as ::core::ffi::c_int,
    145 as ::core::ffi::c_int,
    182 as ::core::ffi::c_int,
    230 as ::core::ffi::c_int,
    290 as ::core::ffi::c_int,
    365 as ::core::ffi::c_int,
    460 as ::core::ffi::c_int,
    580 as ::core::ffi::c_int,
    731 as ::core::ffi::c_int,
    921 as ::core::ffi::c_int,
    1161 as ::core::ffi::c_int,
    1462 as ::core::ffi::c_int,
    1843 as ::core::ffi::c_int,
    2322 as ::core::ffi::c_int,
    2925 as ::core::ffi::c_int,
    3686 as ::core::ffi::c_int,
    4644 as ::core::ffi::c_int,
    5851 as ::core::ffi::c_int,
    7372 as ::core::ffi::c_int,
    9289 as ::core::ffi::c_int,
    11703 as ::core::ffi::c_int,
    14745 as ::core::ffi::c_int,
    18578 as ::core::ffi::c_int,
    23407 as ::core::ffi::c_int,
    29491 as ::core::ffi::c_int,
    37156 as ::core::ffi::c_int,
    46814 as ::core::ffi::c_int,
    58982 as ::core::ffi::c_int,
    74313 as ::core::ffi::c_int,
    93628 as ::core::ffi::c_int,
    117964 as ::core::ffi::c_int,
    148626 as ::core::ffi::c_int,
    187257 as ::core::ffi::c_int,
    235929 as ::core::ffi::c_int,
    297252 as ::core::ffi::c_int,
    374514 as ::core::ffi::c_int,
    471859 as ::core::ffi::c_int,
    594505 as ::core::ffi::c_int,
    749029 as ::core::ffi::c_int,
    943718 as ::core::ffi::c_int,
    1189010 as ::core::ffi::c_int,
    1498059 as ::core::ffi::c_int,
    1887436 as ::core::ffi::c_int,
    2378021 as ::core::ffi::c_int,
    2996119 as ::core::ffi::c_int,
    3774873 as ::core::ffi::c_int,
    4756042 as ::core::ffi::c_int,
    5992238 as ::core::ffi::c_int,
    7549747 as ::core::ffi::c_int,
    9512085 as ::core::ffi::c_int,
    11984476 as ::core::ffi::c_int,
    15099494 as ::core::ffi::c_int,
    19024170 as ::core::ffi::c_int,
    23968953 as ::core::ffi::c_int,
    30198988 as ::core::ffi::c_int,
    38048341 as ::core::ffi::c_int,
    47937906 as ::core::ffi::c_int,
    60397977 as ::core::ffi::c_int,
    76096683 as ::core::ffi::c_int,
    95875813 as ::core::ffi::c_int,
    120795955 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
    134217727 as ::core::ffi::c_int,
];
#[no_mangle]
#[c2rust::src_loc = "131:11"]
pub static mut x264_trellis_lambda2_tab: [[::core::ffi::c_int; 82]; 2] = [
    [
        46 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int,
        92 as ::core::ffi::c_int,
        117 as ::core::ffi::c_int,
        147 as ::core::ffi::c_int,
        185 as ::core::ffi::c_int,
        233 as ::core::ffi::c_int,
        294 as ::core::ffi::c_int,
        370 as ::core::ffi::c_int,
        466 as ::core::ffi::c_int,
        587 as ::core::ffi::c_int,
        740 as ::core::ffi::c_int,
        932 as ::core::ffi::c_int,
        1174 as ::core::ffi::c_int,
        1480 as ::core::ffi::c_int,
        1864 as ::core::ffi::c_int,
        2349 as ::core::ffi::c_int,
        2959 as ::core::ffi::c_int,
        3728 as ::core::ffi::c_int,
        4697 as ::core::ffi::c_int,
        5918 as ::core::ffi::c_int,
        7457 as ::core::ffi::c_int,
        9395 as ::core::ffi::c_int,
        11837 as ::core::ffi::c_int,
        14914 as ::core::ffi::c_int,
        18790 as ::core::ffi::c_int,
        23674 as ::core::ffi::c_int,
        29828 as ::core::ffi::c_int,
        37581 as ::core::ffi::c_int,
        47349 as ::core::ffi::c_int,
        59656 as ::core::ffi::c_int,
        75163 as ::core::ffi::c_int,
        94699 as ::core::ffi::c_int,
        119313 as ::core::ffi::c_int,
        150326 as ::core::ffi::c_int,
        189399 as ::core::ffi::c_int,
        238627 as ::core::ffi::c_int,
        300652 as ::core::ffi::c_int,
        378798 as ::core::ffi::c_int,
        477255 as ::core::ffi::c_int,
        601304 as ::core::ffi::c_int,
        757596 as ::core::ffi::c_int,
        954511 as ::core::ffi::c_int,
        1202608 as ::core::ffi::c_int,
        1515192 as ::core::ffi::c_int,
        1909022 as ::core::ffi::c_int,
        2405217 as ::core::ffi::c_int,
        3030384 as ::core::ffi::c_int,
        3818045 as ::core::ffi::c_int,
        4810435 as ::core::ffi::c_int,
        6060769 as ::core::ffi::c_int,
        7636091 as ::core::ffi::c_int,
        9620872 as ::core::ffi::c_int,
        12121539 as ::core::ffi::c_int,
        15272182 as ::core::ffi::c_int,
        19241743 as ::core::ffi::c_int,
        24243077 as ::core::ffi::c_int,
        30544363 as ::core::ffi::c_int,
        38483486 as ::core::ffi::c_int,
        48486154 as ::core::ffi::c_int,
        61088726 as ::core::ffi::c_int,
        76966972 as ::core::ffi::c_int,
        96972308 as ::core::ffi::c_int,
        122177453 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        27 as ::core::ffi::c_int,
        34 as ::core::ffi::c_int,
        43 as ::core::ffi::c_int,
        54 as ::core::ffi::c_int,
        68 as ::core::ffi::c_int,
        86 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int,
        136 as ::core::ffi::c_int,
        172 as ::core::ffi::c_int,
        216 as ::core::ffi::c_int,
        273 as ::core::ffi::c_int,
        343 as ::core::ffi::c_int,
        433 as ::core::ffi::c_int,
        545 as ::core::ffi::c_int,
        687 as ::core::ffi::c_int,
        865 as ::core::ffi::c_int,
        1090 as ::core::ffi::c_int,
        1374 as ::core::ffi::c_int,
        1731 as ::core::ffi::c_int,
        2180 as ::core::ffi::c_int,
        2747 as ::core::ffi::c_int,
        3461 as ::core::ffi::c_int,
        4361 as ::core::ffi::c_int,
        5494 as ::core::ffi::c_int,
        6922 as ::core::ffi::c_int,
        8721 as ::core::ffi::c_int,
        10988 as ::core::ffi::c_int,
        13844 as ::core::ffi::c_int,
        17442 as ::core::ffi::c_int,
        21976 as ::core::ffi::c_int,
        27688 as ::core::ffi::c_int,
        34885 as ::core::ffi::c_int,
        43953 as ::core::ffi::c_int,
        55377 as ::core::ffi::c_int,
        69771 as ::core::ffi::c_int,
        87906 as ::core::ffi::c_int,
        110755 as ::core::ffi::c_int,
        139543 as ::core::ffi::c_int,
        175813 as ::core::ffi::c_int,
        221511 as ::core::ffi::c_int,
        279087 as ::core::ffi::c_int,
        351627 as ::core::ffi::c_int,
        443023 as ::core::ffi::c_int,
        558174 as ::core::ffi::c_int,
        703255 as ::core::ffi::c_int,
        886046 as ::core::ffi::c_int,
        1116348 as ::core::ffi::c_int,
        1406511 as ::core::ffi::c_int,
        1772093 as ::core::ffi::c_int,
        2232697 as ::core::ffi::c_int,
        2813022 as ::core::ffi::c_int,
        3544186 as ::core::ffi::c_int,
        4465396 as ::core::ffi::c_int,
        5626046 as ::core::ffi::c_int,
        7088374 as ::core::ffi::c_int,
        8930791 as ::core::ffi::c_int,
        11252092 as ::core::ffi::c_int,
        14176748 as ::core::ffi::c_int,
        17861583 as ::core::ffi::c_int,
        22504184 as ::core::ffi::c_int,
        28353495 as ::core::ffi::c_int,
        35723165 as ::core::ffi::c_int,
        45008368 as ::core::ffi::c_int,
        56706990 as ::core::ffi::c_int,
        71446330 as ::core::ffi::c_int,
        90016736 as ::core::ffi::c_int,
        113413980 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
        134217727 as ::core::ffi::c_int,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "168:16"]
pub static mut x264_chroma_lambda2_offset_tab: [uint16_t; 37] = [
    16 as ::core::ffi::c_int as uint16_t,
    20 as ::core::ffi::c_int as uint16_t,
    25 as ::core::ffi::c_int as uint16_t,
    32 as ::core::ffi::c_int as uint16_t,
    40 as ::core::ffi::c_int as uint16_t,
    50 as ::core::ffi::c_int as uint16_t,
    64 as ::core::ffi::c_int as uint16_t,
    80 as ::core::ffi::c_int as uint16_t,
    101 as ::core::ffi::c_int as uint16_t,
    128 as ::core::ffi::c_int as uint16_t,
    161 as ::core::ffi::c_int as uint16_t,
    203 as ::core::ffi::c_int as uint16_t,
    256 as ::core::ffi::c_int as uint16_t,
    322 as ::core::ffi::c_int as uint16_t,
    406 as ::core::ffi::c_int as uint16_t,
    512 as ::core::ffi::c_int as uint16_t,
    645 as ::core::ffi::c_int as uint16_t,
    812 as ::core::ffi::c_int as uint16_t,
    1024 as ::core::ffi::c_int as uint16_t,
    1290 as ::core::ffi::c_int as uint16_t,
    1625 as ::core::ffi::c_int as uint16_t,
    2048 as ::core::ffi::c_int as uint16_t,
    2580 as ::core::ffi::c_int as uint16_t,
    3250 as ::core::ffi::c_int as uint16_t,
    4096 as ::core::ffi::c_int as uint16_t,
    5160 as ::core::ffi::c_int as uint16_t,
    6501 as ::core::ffi::c_int as uint16_t,
    8192 as ::core::ffi::c_int as uint16_t,
    10321 as ::core::ffi::c_int as uint16_t,
    13003 as ::core::ffi::c_int as uint16_t,
    16384 as ::core::ffi::c_int as uint16_t,
    20642 as ::core::ffi::c_int as uint16_t,
    26007 as ::core::ffi::c_int as uint16_t,
    32768 as ::core::ffi::c_int as uint16_t,
    41285 as ::core::ffi::c_int as uint16_t,
    52015 as ::core::ffi::c_int as uint16_t,
    65535 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "183:15"]
pub static mut x264_hpel_ref0: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "184:15"]
pub static mut x264_hpel_ref1: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "191:15"]
pub static mut x264_cqm_jvt4i: [uint8_t; 16] = [
    6 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    37 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "198:15"]
pub static mut x264_cqm_jvt4p: [uint8_t; 16] = [
    10 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "205:15"]
pub static mut x264_cqm_jvt8i: [uint8_t; 64] = [
    6 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    29 as ::core::ffi::c_int as uint8_t,
    31 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "216:15"]
pub static mut x264_cqm_jvt8p: [uint8_t; 64] = [
    9 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    35 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "227:15"]
pub static mut x264_cqm_flat16: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "238:23"]
pub static mut x264_cqm_jvt: [*const uint8_t; 8] = unsafe {
    [
        x264_cqm_jvt4i.as_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        x264_cqm_jvt8p.as_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        x264_cqm_jvt8p.as_ptr(),
    ]
};
#[no_mangle]
#[c2rust::src_loc = "247:15"]
pub static mut x264_cqm_avci50_4ic: [uint8_t; 16] = [
    16 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    28 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    48 as ::core::ffi::c_int as uint8_t,
    40 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    48 as ::core::ffi::c_int as uint8_t,
    60 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "256:15"]
pub static mut x264_cqm_avci50_p_8iy: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    96 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "269:15"]
pub static mut x264_cqm_avci50_1080i_8iy: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    90 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    93 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    78 as ::core::ffi::c_int as uint8_t,
    81 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
    87 as ::core::ffi::c_int as uint8_t,
    96 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "282:15"]
pub static mut x264_cqm_avci100_720p_4ic: [uint8_t; 16] = [
    16 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
    27 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    41 as ::core::ffi::c_int as uint8_t,
    46 as ::core::ffi::c_int as uint8_t,
    54 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "291:15"]
pub static mut x264_cqm_avci100_720p_8iy: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    21 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    25 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    34 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "304:15"]
pub static mut x264_cqm_avci100_1080_4ic: [uint8_t; 16] = [
    16 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    38 as ::core::ffi::c_int as uint8_t,
    44 as ::core::ffi::c_int as uint8_t,
    50 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "313:15"]
pub static mut x264_cqm_avci100_1080i_8iy: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "326:15"]
pub static mut x264_cqm_avci100_1080p_8iy: [uint8_t; 64] = [
    16 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    18 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    22 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    23 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    26 as ::core::ffi::c_int as uint8_t,
    32 as ::core::ffi::c_int as uint8_t,
    36 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    59 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    68 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "339:15"]
pub static mut x264_cqm_avci300_2160p_4iy: [uint8_t; 16] = [
    12 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    33 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "348:15"]
pub static mut x264_cqm_avci300_2160p_4ic: [uint8_t; 16] = [
    28 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    39 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    77 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    77 as ::core::ffi::c_int as uint8_t,
    104 as ::core::ffi::c_int as uint8_t,
    67 as ::core::ffi::c_int as uint8_t,
    77 as ::core::ffi::c_int as uint8_t,
    104 as ::core::ffi::c_int as uint8_t,
    133 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "357:15"]
pub static mut x264_cqm_avci300_2160p_8iy: [uint8_t; 64] = [
    12 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    16 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    17 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    19 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    20 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    80 as ::core::ffi::c_int as uint8_t,
    24 as ::core::ffi::c_int as uint8_t,
    30 as ::core::ffi::c_int as uint8_t,
    42 as ::core::ffi::c_int as uint8_t,
    56 as ::core::ffi::c_int as uint8_t,
    72 as ::core::ffi::c_int as uint8_t,
    76 as ::core::ffi::c_int as uint8_t,
    80 as ::core::ffi::c_int as uint8_t,
    84 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "373:15"]
pub static mut x264_decimate_table4: [uint8_t; 16] = [
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "377:15"]
pub static mut x264_decimate_table8: [uint8_t; 64] = [
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "397:16"]
pub static mut x264_dct8_weight_tab: [uint32_t; 64] = [0; 64];
#[no_mangle]
#[c2rust::src_loc = "413:16"]
pub static mut x264_dct4_weight_tab: [uint32_t; 16] = [0; 16];
#[no_mangle]
#[c2rust::src_loc = "425:16"]
pub static mut x264_dct4_weight2_tab: [uint32_t; 16] = [0; 16];
#[no_mangle]
#[c2rust::src_loc = "439:16"]
pub static mut x264_dct8_weight2_tab: [uint32_t; 64] = [0; 64];
#[no_mangle]
#[c2rust::src_loc = "456:14"]
pub static mut x264_cabac_context_init_I: [[int8_t; 2]; 1024] = [
    [
        20 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(28 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(23 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        53 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        41 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        41 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        102 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(21 as ::core::ffi::c_int) as int8_t,
        107 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(27 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(31 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        95 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(27 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(21 as ::core::ffi::c_int) as int8_t,
        114 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        123 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        122 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(20 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(22 as ::core::ffi::c_int) as int8_t,
        125 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        52 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        108 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        100 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        101 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(19 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        102 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        100 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        95 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        37 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        57 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        25 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        37 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        30 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        41 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        49 as ::core::ffi::c_int as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(19 as ::core::ffi::c_int) as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        -(25 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(20 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(27 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(28 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
    ],
    [
        27 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        22 as ::core::ffi::c_int as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(16 as ::core::ffi::c_int) as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        -(4 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        -(3 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        5 as ::core::ffi::c_int as int8_t,
    ],
    [
        42 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        39 as ::core::ffi::c_int as int8_t,
        22 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        48 as ::core::ffi::c_int as int8_t,
    ],
    [
        27 as ::core::ffi::c_int as int8_t,
        37 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        110 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(20 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        96 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(19 as ::core::ffi::c_int) as int8_t,
        117 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        48 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        53 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        32 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        48 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        96 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        99 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        95 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        95 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        14 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        15 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        26 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        47 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        21 as ::core::ffi::c_int as int8_t,
    ],
    [
        25 as ::core::ffi::c_int as int8_t,
        22 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        57 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        109 as ::core::ffi::c_int as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(35 as ::core::ffi::c_int) as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(34 as ::core::ffi::c_int) as int8_t,
    ],
    [
        32 as ::core::ffi::c_int as int8_t,
        -(26 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(30 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        -(32 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(18 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(7 as ::core::ffi::c_int) as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        -(5 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        2 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        -(3 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        30 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        -(7 as ::core::ffi::c_int) as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(3 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        5 as ::core::ffi::c_int as int8_t,
    ],
    [
        32 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        5 as ::core::ffi::c_int as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        12 as ::core::ffi::c_int as int8_t,
    ],
    [
        39 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        26 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        21 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        31 as ::core::ffi::c_int as int8_t,
    ],
    [
        25 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        120 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(20 as ::core::ffi::c_int) as int8_t,
        112 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        114 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(26 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        81 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(23 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        49 as ::core::ffi::c_int as int8_t,
        -(14 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        45 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        43 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        106 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        -(10 as ::core::ffi::c_int) as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        123 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        122 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        123 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        122 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        115 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        52 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        108 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        100 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        101 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(19 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        52 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        108 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        100 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        101 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        87 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(19 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        25 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        37 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        30 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        41 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        49 as ::core::ffi::c_int as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(19 as ::core::ffi::c_int) as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        -(25 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(20 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(27 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(28 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        8 as ::core::ffi::c_int as int8_t,
        25 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        37 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        18 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        33 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        30 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        38 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        11 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        45 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        41 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        49 as ::core::ffi::c_int as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        51 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(19 as ::core::ffi::c_int) as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        30 as ::core::ffi::c_int as int8_t,
        -(25 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(20 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(27 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(23 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(28 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        -(17 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        -(6 as ::core::ffi::c_int) as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
    [
        41 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        120 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(20 as ::core::ffi::c_int) as int8_t,
        112 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        114 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(26 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        81 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(23 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        106 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        49 as ::core::ffi::c_int as int8_t,
        -(14 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        45 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        -(10 as ::core::ffi::c_int) as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        43 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        120 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(20 as ::core::ffi::c_int) as int8_t,
        112 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        114 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        89 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(26 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        81 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        56 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(23 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(24 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        106 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(18 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(14 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        26 as ::core::ffi::c_int as int8_t,
        -(13 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        49 as ::core::ffi::c_int as int8_t,
        -(14 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        45 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        -(10 as ::core::ffi::c_int) as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        -(11 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(8 as ::core::ffi::c_int) as int8_t,
    ],
    [
        28 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        3 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        9 as ::core::ffi::c_int as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        29 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        34 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        43 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        61 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        32 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        48 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        96 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        84 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        59 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        75 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        79 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        3 as ::core::ffi::c_int as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        16 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        20 as ::core::ffi::c_int as int8_t,
        36 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        32 as ::core::ffi::c_int as int8_t,
    ],
    [
        5 as ::core::ffi::c_int as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        48 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        104 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(11 as ::core::ffi::c_int) as int8_t,
        97 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        96 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        85 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(13 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        4 as ::core::ffi::c_int as int8_t,
        66 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        83 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        14 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        15 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        26 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        47 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        21 as ::core::ffi::c_int as int8_t,
    ],
    [
        25 as ::core::ffi::c_int as int8_t,
        22 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        57 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        109 as ::core::ffi::c_int as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(35 as ::core::ffi::c_int) as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(34 as ::core::ffi::c_int) as int8_t,
    ],
    [
        32 as ::core::ffi::c_int as int8_t,
        -(26 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(30 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        -(32 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(18 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(7 as ::core::ffi::c_int) as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        -(5 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        2 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        6 as ::core::ffi::c_int as int8_t,
    ],
    [
        6 as ::core::ffi::c_int as int8_t,
        19 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        14 as ::core::ffi::c_int as int8_t,
    ],
    [
        18 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        11 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        15 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        16 as ::core::ffi::c_int as int8_t,
    ],
    [
        12 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        23 as ::core::ffi::c_int as int8_t,
    ],
    [
        15 as ::core::ffi::c_int as int8_t,
        20 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        26 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        40 as ::core::ffi::c_int as int8_t,
    ],
    [
        17 as ::core::ffi::c_int as int8_t,
        47 as ::core::ffi::c_int as int8_t,
    ],
    [
        24 as ::core::ffi::c_int as int8_t,
        17 as ::core::ffi::c_int as int8_t,
    ],
    [
        21 as ::core::ffi::c_int as int8_t,
        21 as ::core::ffi::c_int as int8_t,
    ],
    [
        25 as ::core::ffi::c_int as int8_t,
        22 as ::core::ffi::c_int as int8_t,
    ],
    [
        31 as ::core::ffi::c_int as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        22 as ::core::ffi::c_int as int8_t,
        29 as ::core::ffi::c_int as int8_t,
    ],
    [
        19 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        14 as ::core::ffi::c_int as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        10 as ::core::ffi::c_int as int8_t,
        57 as ::core::ffi::c_int as int8_t,
    ],
    [
        7 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        82 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        94 as ::core::ffi::c_int as int8_t,
    ],
    [
        9 as ::core::ffi::c_int as int8_t,
        69 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        109 as ::core::ffi::c_int as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(35 as ::core::ffi::c_int) as int8_t,
    ],
    [
        36 as ::core::ffi::c_int as int8_t,
        -(34 as ::core::ffi::c_int) as int8_t,
    ],
    [
        32 as ::core::ffi::c_int as int8_t,
        -(26 as ::core::ffi::c_int) as int8_t,
    ],
    [
        37 as ::core::ffi::c_int as int8_t,
        -(30 as ::core::ffi::c_int) as int8_t,
    ],
    [
        44 as ::core::ffi::c_int as int8_t,
        -(32 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(18 as ::core::ffi::c_int) as int8_t,
    ],
    [
        34 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        40 as ::core::ffi::c_int as int8_t,
        -(15 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        -(7 as ::core::ffi::c_int) as int8_t,
    ],
    [
        35 as ::core::ffi::c_int as int8_t,
        -(5 as ::core::ffi::c_int) as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        38 as ::core::ffi::c_int as int8_t,
        2 as ::core::ffi::c_int as int8_t,
    ],
    [
        33 as ::core::ffi::c_int as int8_t,
        13 as ::core::ffi::c_int as int8_t,
    ],
    [
        23 as ::core::ffi::c_int as int8_t,
        35 as ::core::ffi::c_int as int8_t,
    ],
    [
        13 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        110 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        71 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        42 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        50 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        54 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        58 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        63 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        72 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        74 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        91 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        67 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(5 as ::core::ffi::c_int) as int8_t,
        27 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        39 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        44 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        46 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(16 as ::core::ffi::c_int) as int8_t,
        64 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        68 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        78 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        77 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        86 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        92 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(15 as ::core::ffi::c_int) as int8_t,
        55 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        60 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(6 as ::core::ffi::c_int) as int8_t,
        62 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(4 as ::core::ffi::c_int) as int8_t,
        65 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(12 as ::core::ffi::c_int) as int8_t,
        73 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        76 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(7 as ::core::ffi::c_int) as int8_t,
        80 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(9 as ::core::ffi::c_int) as int8_t,
        88 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(17 as ::core::ffi::c_int) as int8_t,
        110 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(3 as ::core::ffi::c_int) as int8_t,
        70 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(8 as ::core::ffi::c_int) as int8_t,
        93 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(10 as ::core::ffi::c_int) as int8_t,
        90 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(30 as ::core::ffi::c_int) as int8_t,
        127 as ::core::ffi::c_int as int8_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "768:14"]
pub static mut x264_cabac_context_init_PB: [[[int8_t; 2]; 1024]; 3] = [
    [
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            2 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(46 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            4 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            5 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(23 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(31 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            -(45 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(26 as ::core::ffi::c_int) as int8_t,
        ],
        [
            65 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            125 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            8 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            2 as ::core::ffi::c_int as int8_t,
        ],
        [
            60 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            4 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            4 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            2 as ::core::ffi::c_int as int8_t,
        ],
        [
            60 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(7 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            2 as ::core::ffi::c_int as int8_t,
        ],
        [
            60 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(23 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(31 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            -(45 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(26 as ::core::ffi::c_int) as int8_t,
        ],
        [
            65 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            125 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            8 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(23 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(31 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            -(45 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(26 as ::core::ffi::c_int) as int8_t,
        ],
        [
            65 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            125 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            6 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            8 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            57 as ::core::ffi::c_int as int8_t,
            2 as ::core::ffi::c_int as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(45 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(39 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(36 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(78 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(70 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(46 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(44 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            46 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(11 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            102 as ::core::ffi::c_int as int8_t,
            -(94 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(58 as ::core::ffi::c_int) as int8_t,
        ],
        [
            67 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            44 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(27 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(8 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            116 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            4 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            8 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            8 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            19 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            121 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            116 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(33 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            -(42 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(71 as ::core::ffi::c_int) as int8_t,
        ],
        [
            63 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            -(64 as ::core::ffi::c_int) as int8_t,
        ],
        [
            77 as ::core::ffi::c_int as int8_t,
            -(74 as ::core::ffi::c_int) as int8_t,
        ],
        [
            54 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(35 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(37 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            21 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            32 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(35 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(78 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(70 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(46 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(44 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(78 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(70 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(46 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            119 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(44 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(11 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            102 as ::core::ffi::c_int as int8_t,
            -(94 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(58 as ::core::ffi::c_int) as int8_t,
        ],
        [
            67 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            44 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(27 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(8 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(11 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(43 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            102 as ::core::ffi::c_int as int8_t,
            -(94 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            51 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(58 as ::core::ffi::c_int) as int8_t,
        ],
        [
            67 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            44 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            55 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(27 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(8 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            53 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            64 as ::core::ffi::c_int as int8_t,
            3 as ::core::ffi::c_int as int8_t,
        ],
        [
            68 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            121 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            116 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(33 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            110 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            126 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            105 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            121 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            116 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(33 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            -(42 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(71 as ::core::ffi::c_int) as int8_t,
        ],
        [
            63 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            -(64 as ::core::ffi::c_int) as int8_t,
        ],
        [
            77 as ::core::ffi::c_int as int8_t,
            -(74 as ::core::ffi::c_int) as int8_t,
        ],
        [
            54 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(35 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(37 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            -(6 as ::core::ffi::c_int) as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(22 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(28 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            47 as ::core::ffi::c_int as int8_t,
            -(42 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            9 as ::core::ffi::c_int as int8_t,
        ],
        [
            69 as ::core::ffi::c_int as int8_t,
            -(71 as ::core::ffi::c_int) as int8_t,
        ],
        [
            63 as ::core::ffi::c_int as int8_t,
            -(63 as ::core::ffi::c_int) as int8_t,
        ],
        [
            66 as ::core::ffi::c_int as int8_t,
            -(64 as ::core::ffi::c_int) as int8_t,
        ],
        [
            77 as ::core::ffi::c_int as int8_t,
            -(74 as ::core::ffi::c_int) as int8_t,
        ],
        [
            54 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(35 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            14 as ::core::ffi::c_int as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            23 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(36 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(37 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(33 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            49 as ::core::ffi::c_int as int8_t,
            -(29 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            40 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            38 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(5 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            112 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(34 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            107 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            111 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            122 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            -(15 as ::core::ffi::c_int) as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            54 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(32 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            116 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            96 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            101 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            97 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(36 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            95 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            103 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(20 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            44 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            50 as ::core::ffi::c_int as int8_t,
            -(34 as ::core::ffi::c_int) as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(44 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            21 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            93 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            26 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            115 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            87 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            113 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(47 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(32 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(51 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            -(55 as ::core::ffi::c_int) as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(46 as ::core::ffi::c_int) as int8_t,
        ],
        [
            62 as ::core::ffi::c_int as int8_t,
            -(50 as ::core::ffi::c_int) as int8_t,
        ],
        [
            81 as ::core::ffi::c_int as int8_t,
            -(67 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(20 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            26 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            24 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            30 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            123 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            25 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            50 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            84 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            51 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(4 as ::core::ffi::c_int) as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            76 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            90 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            80 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            35 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            27 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(44 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            39 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            34 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            31 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            40 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            43 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            36 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            3 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            6 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            11 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            42 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            13 as ::core::ffi::c_int as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            58 as ::core::ffi::c_int as int8_t,
        ],
        [
            10 as ::core::ffi::c_int as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            12 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            69 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            8 as ::core::ffi::c_int as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(18 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(25 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            -(3 as ::core::ffi::c_int) as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            10 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(44 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(24 as ::core::ffi::c_int) as int8_t,
        ],
        [
            19 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            29 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            33 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            20 as ::core::ffi::c_int as int8_t,
        ],
        [
            22 as ::core::ffi::c_int as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(3 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            74 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(17 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            72 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            52 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            9 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            -(10 as ::core::ffi::c_int) as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            -(4 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            7 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            12 as ::core::ffi::c_int as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            23 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            37 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            49 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            56 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            63 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(6 as ::core::ffi::c_int) as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            113 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(27 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(19 as ::core::ffi::c_int) as int8_t,
            114 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(25 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(26 as ::core::ffi::c_int) as int8_t,
            117 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            113 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(28 as ::core::ffi::c_int) as int8_t,
            118 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(31 as ::core::ffi::c_int) as int8_t,
            120 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            124 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            102 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            99 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            106 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(50 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            86 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(2 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            73 as ::core::ffi::c_int as int8_t,
        ],
        [
            4 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            81 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            64 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            68 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            78 as ::core::ffi::c_int as int8_t,
        ],
        [
            7 as ::core::ffi::c_int as int8_t,
            55 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            65 as ::core::ffi::c_int as int8_t,
        ],
        [
            14 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            44 as ::core::ffi::c_int as int8_t,
        ],
        [
            5 as ::core::ffi::c_int as int8_t,
            60 as ::core::ffi::c_int as int8_t,
        ],
        [
            2 as ::core::ffi::c_int as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(47 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(32 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(51 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            -(55 as ::core::ffi::c_int) as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(46 as ::core::ffi::c_int) as int8_t,
        ],
        [
            62 as ::core::ffi::c_int as int8_t,
            -(50 as ::core::ffi::c_int) as int8_t,
        ],
        [
            81 as ::core::ffi::c_int as int8_t,
            -(67 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(20 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(13 as ::core::ffi::c_int) as int8_t,
        ],
        [
            16 as ::core::ffi::c_int as int8_t,
            -(9 as ::core::ffi::c_int) as int8_t,
        ],
        [
            17 as ::core::ffi::c_int as int8_t,
            -(12 as ::core::ffi::c_int) as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            -(21 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(30 as ::core::ffi::c_int) as int8_t,
        ],
        [
            41 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            42 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            48 as ::core::ffi::c_int as int8_t,
            -(47 as ::core::ffi::c_int) as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            -(32 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(40 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(51 as ::core::ffi::c_int) as int8_t,
        ],
        [
            46 as ::core::ffi::c_int as int8_t,
            -(41 as ::core::ffi::c_int) as int8_t,
        ],
        [
            52 as ::core::ffi::c_int as int8_t,
            -(39 as ::core::ffi::c_int) as int8_t,
        ],
        [
            43 as ::core::ffi::c_int as int8_t,
            -(19 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            11 as ::core::ffi::c_int as int8_t,
        ],
        [
            61 as ::core::ffi::c_int as int8_t,
            -(55 as ::core::ffi::c_int) as int8_t,
        ],
        [
            56 as ::core::ffi::c_int as int8_t,
            -(46 as ::core::ffi::c_int) as int8_t,
        ],
        [
            62 as ::core::ffi::c_int as int8_t,
            -(50 as ::core::ffi::c_int) as int8_t,
        ],
        [
            81 as ::core::ffi::c_int as int8_t,
            -(67 as ::core::ffi::c_int) as int8_t,
        ],
        [
            45 as ::core::ffi::c_int as int8_t,
            -(20 as ::core::ffi::c_int) as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            -(2 as ::core::ffi::c_int) as int8_t,
        ],
        [
            28 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            39 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            30 as ::core::ffi::c_int as int8_t,
            17 as ::core::ffi::c_int as int8_t,
        ],
        [
            20 as ::core::ffi::c_int as int8_t,
            38 as ::core::ffi::c_int as int8_t,
        ],
        [
            18 as ::core::ffi::c_int as int8_t,
            45 as ::core::ffi::c_int as int8_t,
        ],
        [
            15 as ::core::ffi::c_int as int8_t,
            54 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            -(16 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(14 as ::core::ffi::c_int) as int8_t,
        ],
        [
            37 as ::core::ffi::c_int as int8_t,
            -(17 as ::core::ffi::c_int) as int8_t,
        ],
        [
            32 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            29 as ::core::ffi::c_int as int8_t,
            15 as ::core::ffi::c_int as int8_t,
        ],
        [
            24 as ::core::ffi::c_int as int8_t,
            25 as ::core::ffi::c_int as int8_t,
        ],
        [
            34 as ::core::ffi::c_int as int8_t,
            22 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            16 as ::core::ffi::c_int as int8_t,
        ],
        [
            35 as ::core::ffi::c_int as int8_t,
            18 as ::core::ffi::c_int as int8_t,
        ],
        [
            31 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            33 as ::core::ffi::c_int as int8_t,
            41 as ::core::ffi::c_int as int8_t,
        ],
        [
            36 as ::core::ffi::c_int as int8_t,
            28 as ::core::ffi::c_int as int8_t,
        ],
        [
            27 as ::core::ffi::c_int as int8_t,
            47 as ::core::ffi::c_int as int8_t,
        ],
        [
            21 as ::core::ffi::c_int as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            115 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(24 as ::core::ffi::c_int) as int8_t,
            115 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(22 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            62 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            53 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            59 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            89 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(13 as ::core::ffi::c_int) as int8_t,
            94 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(29 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            100 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            57 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            67 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            71 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            77 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(21 as ::core::ffi::c_int) as int8_t,
            85 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(16 as ::core::ffi::c_int) as int8_t,
            88 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(23 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(15 as ::core::ffi::c_int) as int8_t,
            98 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(37 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            82 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            48 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            61 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(8 as ::core::ffi::c_int) as int8_t,
            66 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(7 as ::core::ffi::c_int) as int8_t,
            70 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(14 as ::core::ffi::c_int) as int8_t,
            75 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(10 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(9 as ::core::ffi::c_int) as int8_t,
            83 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(12 as ::core::ffi::c_int) as int8_t,
            92 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(18 as ::core::ffi::c_int) as int8_t,
            108 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(5 as ::core::ffi::c_int) as int8_t,
            79 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            104 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(11 as ::core::ffi::c_int) as int8_t,
            91 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(30 as ::core::ffi::c_int) as int8_t,
            127 as ::core::ffi::c_int as int8_t,
        ],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1668:15"]
pub static mut x264_cabac_range_lps: [[uint8_t; 4]; 64] = [
    [
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
    ],
    [
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
    ],
    [
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
    ],
    [
        6 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
    ],
    [
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
    ],
    [
        7 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
    ],
    [
        7 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
    ],
    [
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
    ],
    [
        8 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
    ],
    [
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
    ],
    [
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ],
    [
        10 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
    ],
    [
        10 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
    ],
    [
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
    ],
    [
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
    ],
    [
        12 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
    ],
    [
        12 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
    ],
    [
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
    ],
    [
        15 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
    ],
    [
        16 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
    ],
    [
        17 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
    ],
    [
        18 as ::core::ffi::c_int as uint8_t,
        22 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
    ],
    [
        19 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
    ],
    [
        20 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
    ],
    [
        21 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
    ],
    [
        22 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
    ],
    [
        23 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
    ],
    [
        24 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
    ],
    [
        26 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
    ],
    [
        27 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
    ],
    [
        29 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
    ],
    [
        30 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
    ],
    [
        32 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
    ],
    [
        33 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
    ],
    [
        35 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
    ],
    [
        37 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
    ],
    [
        39 as ::core::ffi::c_int as uint8_t,
        48 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        65 as ::core::ffi::c_int as uint8_t,
    ],
    [
        41 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        69 as ::core::ffi::c_int as uint8_t,
    ],
    [
        43 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
        72 as ::core::ffi::c_int as uint8_t,
    ],
    [
        46 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
        66 as ::core::ffi::c_int as uint8_t,
        76 as ::core::ffi::c_int as uint8_t,
    ],
    [
        48 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
        69 as ::core::ffi::c_int as uint8_t,
        80 as ::core::ffi::c_int as uint8_t,
    ],
    [
        51 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
        73 as ::core::ffi::c_int as uint8_t,
        85 as ::core::ffi::c_int as uint8_t,
    ],
    [
        53 as ::core::ffi::c_int as uint8_t,
        65 as ::core::ffi::c_int as uint8_t,
        77 as ::core::ffi::c_int as uint8_t,
        89 as ::core::ffi::c_int as uint8_t,
    ],
    [
        56 as ::core::ffi::c_int as uint8_t,
        69 as ::core::ffi::c_int as uint8_t,
        81 as ::core::ffi::c_int as uint8_t,
        94 as ::core::ffi::c_int as uint8_t,
    ],
    [
        59 as ::core::ffi::c_int as uint8_t,
        72 as ::core::ffi::c_int as uint8_t,
        86 as ::core::ffi::c_int as uint8_t,
        99 as ::core::ffi::c_int as uint8_t,
    ],
    [
        62 as ::core::ffi::c_int as uint8_t,
        76 as ::core::ffi::c_int as uint8_t,
        90 as ::core::ffi::c_int as uint8_t,
        104 as ::core::ffi::c_int as uint8_t,
    ],
    [
        66 as ::core::ffi::c_int as uint8_t,
        80 as ::core::ffi::c_int as uint8_t,
        95 as ::core::ffi::c_int as uint8_t,
        110 as ::core::ffi::c_int as uint8_t,
    ],
    [
        69 as ::core::ffi::c_int as uint8_t,
        85 as ::core::ffi::c_int as uint8_t,
        100 as ::core::ffi::c_int as uint8_t,
        116 as ::core::ffi::c_int as uint8_t,
    ],
    [
        73 as ::core::ffi::c_int as uint8_t,
        89 as ::core::ffi::c_int as uint8_t,
        105 as ::core::ffi::c_int as uint8_t,
        122 as ::core::ffi::c_int as uint8_t,
    ],
    [
        77 as ::core::ffi::c_int as uint8_t,
        94 as ::core::ffi::c_int as uint8_t,
        111 as ::core::ffi::c_int as uint8_t,
        128 as ::core::ffi::c_int as uint8_t,
    ],
    [
        81 as ::core::ffi::c_int as uint8_t,
        99 as ::core::ffi::c_int as uint8_t,
        117 as ::core::ffi::c_int as uint8_t,
        135 as ::core::ffi::c_int as uint8_t,
    ],
    [
        85 as ::core::ffi::c_int as uint8_t,
        104 as ::core::ffi::c_int as uint8_t,
        123 as ::core::ffi::c_int as uint8_t,
        142 as ::core::ffi::c_int as uint8_t,
    ],
    [
        90 as ::core::ffi::c_int as uint8_t,
        110 as ::core::ffi::c_int as uint8_t,
        130 as ::core::ffi::c_int as uint8_t,
        150 as ::core::ffi::c_int as uint8_t,
    ],
    [
        95 as ::core::ffi::c_int as uint8_t,
        116 as ::core::ffi::c_int as uint8_t,
        137 as ::core::ffi::c_int as uint8_t,
        158 as ::core::ffi::c_int as uint8_t,
    ],
    [
        100 as ::core::ffi::c_int as uint8_t,
        122 as ::core::ffi::c_int as uint8_t,
        144 as ::core::ffi::c_int as uint8_t,
        166 as ::core::ffi::c_int as uint8_t,
    ],
    [
        105 as ::core::ffi::c_int as uint8_t,
        128 as ::core::ffi::c_int as uint8_t,
        152 as ::core::ffi::c_int as uint8_t,
        175 as ::core::ffi::c_int as uint8_t,
    ],
    [
        111 as ::core::ffi::c_int as uint8_t,
        135 as ::core::ffi::c_int as uint8_t,
        160 as ::core::ffi::c_int as uint8_t,
        185 as ::core::ffi::c_int as uint8_t,
    ],
    [
        116 as ::core::ffi::c_int as uint8_t,
        142 as ::core::ffi::c_int as uint8_t,
        169 as ::core::ffi::c_int as uint8_t,
        195 as ::core::ffi::c_int as uint8_t,
    ],
    [
        123 as ::core::ffi::c_int as uint8_t,
        150 as ::core::ffi::c_int as uint8_t,
        178 as ::core::ffi::c_int as uint8_t,
        205 as ::core::ffi::c_int as uint8_t,
    ],
    [
        128 as ::core::ffi::c_int as uint8_t,
        158 as ::core::ffi::c_int as uint8_t,
        187 as ::core::ffi::c_int as uint8_t,
        216 as ::core::ffi::c_int as uint8_t,
    ],
    [
        128 as ::core::ffi::c_int as uint8_t,
        167 as ::core::ffi::c_int as uint8_t,
        197 as ::core::ffi::c_int as uint8_t,
        227 as ::core::ffi::c_int as uint8_t,
    ],
    [
        128 as ::core::ffi::c_int as uint8_t,
        176 as ::core::ffi::c_int as uint8_t,
        208 as ::core::ffi::c_int as uint8_t,
        240 as ::core::ffi::c_int as uint8_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1688:15"]
pub static mut x264_cabac_transition: [[uint8_t; 2]; 128] = [
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
    ],
    [
        2 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
    ],
    [
        51 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
    ],
    [
        2 as ::core::ffi::c_int as uint8_t,
        50 as ::core::ffi::c_int as uint8_t,
    ],
    [
        51 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
    ],
    [
        4 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
    ],
    [
        53 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
    ],
    [
        6 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
    ],
    [
        53 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
    ],
    [
        8 as ::core::ffi::c_int as uint8_t,
        52 as ::core::ffi::c_int as uint8_t,
    ],
    [
        53 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
    ],
    [
        10 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
    ],
    [
        55 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
    ],
    [
        12 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
    ],
    [
        55 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as uint8_t,
        54 as ::core::ffi::c_int as uint8_t,
    ],
    [
        55 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ],
    [
        16 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
    ],
    [
        57 as ::core::ffi::c_int as uint8_t,
        17 as ::core::ffi::c_int as uint8_t,
    ],
    [
        18 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
    ],
    [
        57 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
    ],
    [
        20 as ::core::ffi::c_int as uint8_t,
        56 as ::core::ffi::c_int as uint8_t,
    ],
    [
        57 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
    ],
    [
        22 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
    ],
    [
        59 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
    ],
    [
        24 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
    ],
    [
        59 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
    ],
    [
        26 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
    ],
    [
        61 as ::core::ffi::c_int as uint8_t,
        27 as ::core::ffi::c_int as uint8_t,
    ],
    [
        28 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
    ],
    [
        61 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
    ],
    [
        30 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
    ],
    [
        61 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
    ],
    [
        32 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
    ],
    [
        63 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
    ],
    [
        34 as ::core::ffi::c_int as uint8_t,
        62 as ::core::ffi::c_int as uint8_t,
    ],
    [
        63 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
    ],
    [
        36 as ::core::ffi::c_int as uint8_t,
        64 as ::core::ffi::c_int as uint8_t,
    ],
    [
        65 as ::core::ffi::c_int as uint8_t,
        37 as ::core::ffi::c_int as uint8_t,
    ],
    [
        38 as ::core::ffi::c_int as uint8_t,
        66 as ::core::ffi::c_int as uint8_t,
    ],
    [
        67 as ::core::ffi::c_int as uint8_t,
        39 as ::core::ffi::c_int as uint8_t,
    ],
    [
        40 as ::core::ffi::c_int as uint8_t,
        66 as ::core::ffi::c_int as uint8_t,
    ],
    [
        67 as ::core::ffi::c_int as uint8_t,
        41 as ::core::ffi::c_int as uint8_t,
    ],
    [
        42 as ::core::ffi::c_int as uint8_t,
        66 as ::core::ffi::c_int as uint8_t,
    ],
    [
        67 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
    ],
    [
        44 as ::core::ffi::c_int as uint8_t,
        68 as ::core::ffi::c_int as uint8_t,
    ],
    [
        69 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
    ],
    [
        46 as ::core::ffi::c_int as uint8_t,
        68 as ::core::ffi::c_int as uint8_t,
    ],
    [
        69 as ::core::ffi::c_int as uint8_t,
        47 as ::core::ffi::c_int as uint8_t,
    ],
    [
        48 as ::core::ffi::c_int as uint8_t,
        70 as ::core::ffi::c_int as uint8_t,
    ],
    [
        71 as ::core::ffi::c_int as uint8_t,
        49 as ::core::ffi::c_int as uint8_t,
    ],
    [
        50 as ::core::ffi::c_int as uint8_t,
        72 as ::core::ffi::c_int as uint8_t,
    ],
    [
        73 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
    ],
    [
        52 as ::core::ffi::c_int as uint8_t,
        72 as ::core::ffi::c_int as uint8_t,
    ],
    [
        73 as ::core::ffi::c_int as uint8_t,
        53 as ::core::ffi::c_int as uint8_t,
    ],
    [
        54 as ::core::ffi::c_int as uint8_t,
        74 as ::core::ffi::c_int as uint8_t,
    ],
    [
        75 as ::core::ffi::c_int as uint8_t,
        55 as ::core::ffi::c_int as uint8_t,
    ],
    [
        56 as ::core::ffi::c_int as uint8_t,
        74 as ::core::ffi::c_int as uint8_t,
    ],
    [
        75 as ::core::ffi::c_int as uint8_t,
        57 as ::core::ffi::c_int as uint8_t,
    ],
    [
        58 as ::core::ffi::c_int as uint8_t,
        76 as ::core::ffi::c_int as uint8_t,
    ],
    [
        77 as ::core::ffi::c_int as uint8_t,
        59 as ::core::ffi::c_int as uint8_t,
    ],
    [
        60 as ::core::ffi::c_int as uint8_t,
        78 as ::core::ffi::c_int as uint8_t,
    ],
    [
        79 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
    ],
    [
        62 as ::core::ffi::c_int as uint8_t,
        78 as ::core::ffi::c_int as uint8_t,
    ],
    [
        79 as ::core::ffi::c_int as uint8_t,
        63 as ::core::ffi::c_int as uint8_t,
    ],
    [
        64 as ::core::ffi::c_int as uint8_t,
        80 as ::core::ffi::c_int as uint8_t,
    ],
    [
        81 as ::core::ffi::c_int as uint8_t,
        65 as ::core::ffi::c_int as uint8_t,
    ],
    [
        66 as ::core::ffi::c_int as uint8_t,
        82 as ::core::ffi::c_int as uint8_t,
    ],
    [
        83 as ::core::ffi::c_int as uint8_t,
        67 as ::core::ffi::c_int as uint8_t,
    ],
    [
        68 as ::core::ffi::c_int as uint8_t,
        82 as ::core::ffi::c_int as uint8_t,
    ],
    [
        83 as ::core::ffi::c_int as uint8_t,
        69 as ::core::ffi::c_int as uint8_t,
    ],
    [
        70 as ::core::ffi::c_int as uint8_t,
        84 as ::core::ffi::c_int as uint8_t,
    ],
    [
        85 as ::core::ffi::c_int as uint8_t,
        71 as ::core::ffi::c_int as uint8_t,
    ],
    [
        72 as ::core::ffi::c_int as uint8_t,
        84 as ::core::ffi::c_int as uint8_t,
    ],
    [
        85 as ::core::ffi::c_int as uint8_t,
        73 as ::core::ffi::c_int as uint8_t,
    ],
    [
        74 as ::core::ffi::c_int as uint8_t,
        88 as ::core::ffi::c_int as uint8_t,
    ],
    [
        89 as ::core::ffi::c_int as uint8_t,
        75 as ::core::ffi::c_int as uint8_t,
    ],
    [
        76 as ::core::ffi::c_int as uint8_t,
        88 as ::core::ffi::c_int as uint8_t,
    ],
    [
        89 as ::core::ffi::c_int as uint8_t,
        77 as ::core::ffi::c_int as uint8_t,
    ],
    [
        78 as ::core::ffi::c_int as uint8_t,
        90 as ::core::ffi::c_int as uint8_t,
    ],
    [
        91 as ::core::ffi::c_int as uint8_t,
        79 as ::core::ffi::c_int as uint8_t,
    ],
    [
        80 as ::core::ffi::c_int as uint8_t,
        90 as ::core::ffi::c_int as uint8_t,
    ],
    [
        91 as ::core::ffi::c_int as uint8_t,
        81 as ::core::ffi::c_int as uint8_t,
    ],
    [
        82 as ::core::ffi::c_int as uint8_t,
        94 as ::core::ffi::c_int as uint8_t,
    ],
    [
        95 as ::core::ffi::c_int as uint8_t,
        83 as ::core::ffi::c_int as uint8_t,
    ],
    [
        84 as ::core::ffi::c_int as uint8_t,
        94 as ::core::ffi::c_int as uint8_t,
    ],
    [
        95 as ::core::ffi::c_int as uint8_t,
        85 as ::core::ffi::c_int as uint8_t,
    ],
    [
        86 as ::core::ffi::c_int as uint8_t,
        96 as ::core::ffi::c_int as uint8_t,
    ],
    [
        97 as ::core::ffi::c_int as uint8_t,
        87 as ::core::ffi::c_int as uint8_t,
    ],
    [
        88 as ::core::ffi::c_int as uint8_t,
        96 as ::core::ffi::c_int as uint8_t,
    ],
    [
        97 as ::core::ffi::c_int as uint8_t,
        89 as ::core::ffi::c_int as uint8_t,
    ],
    [
        90 as ::core::ffi::c_int as uint8_t,
        100 as ::core::ffi::c_int as uint8_t,
    ],
    [
        101 as ::core::ffi::c_int as uint8_t,
        91 as ::core::ffi::c_int as uint8_t,
    ],
    [
        92 as ::core::ffi::c_int as uint8_t,
        100 as ::core::ffi::c_int as uint8_t,
    ],
    [
        101 as ::core::ffi::c_int as uint8_t,
        93 as ::core::ffi::c_int as uint8_t,
    ],
    [
        94 as ::core::ffi::c_int as uint8_t,
        102 as ::core::ffi::c_int as uint8_t,
    ],
    [
        103 as ::core::ffi::c_int as uint8_t,
        95 as ::core::ffi::c_int as uint8_t,
    ],
    [
        96 as ::core::ffi::c_int as uint8_t,
        104 as ::core::ffi::c_int as uint8_t,
    ],
    [
        105 as ::core::ffi::c_int as uint8_t,
        97 as ::core::ffi::c_int as uint8_t,
    ],
    [
        98 as ::core::ffi::c_int as uint8_t,
        104 as ::core::ffi::c_int as uint8_t,
    ],
    [
        105 as ::core::ffi::c_int as uint8_t,
        99 as ::core::ffi::c_int as uint8_t,
    ],
    [
        100 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
    ],
    [
        109 as ::core::ffi::c_int as uint8_t,
        101 as ::core::ffi::c_int as uint8_t,
    ],
    [
        102 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
    ],
    [
        109 as ::core::ffi::c_int as uint8_t,
        103 as ::core::ffi::c_int as uint8_t,
    ],
    [
        104 as ::core::ffi::c_int as uint8_t,
        110 as ::core::ffi::c_int as uint8_t,
    ],
    [
        111 as ::core::ffi::c_int as uint8_t,
        105 as ::core::ffi::c_int as uint8_t,
    ],
    [
        106 as ::core::ffi::c_int as uint8_t,
        112 as ::core::ffi::c_int as uint8_t,
    ],
    [
        113 as ::core::ffi::c_int as uint8_t,
        107 as ::core::ffi::c_int as uint8_t,
    ],
    [
        108 as ::core::ffi::c_int as uint8_t,
        114 as ::core::ffi::c_int as uint8_t,
    ],
    [
        115 as ::core::ffi::c_int as uint8_t,
        109 as ::core::ffi::c_int as uint8_t,
    ],
    [
        110 as ::core::ffi::c_int as uint8_t,
        116 as ::core::ffi::c_int as uint8_t,
    ],
    [
        117 as ::core::ffi::c_int as uint8_t,
        111 as ::core::ffi::c_int as uint8_t,
    ],
    [
        112 as ::core::ffi::c_int as uint8_t,
        118 as ::core::ffi::c_int as uint8_t,
    ],
    [
        119 as ::core::ffi::c_int as uint8_t,
        113 as ::core::ffi::c_int as uint8_t,
    ],
    [
        114 as ::core::ffi::c_int as uint8_t,
        118 as ::core::ffi::c_int as uint8_t,
    ],
    [
        119 as ::core::ffi::c_int as uint8_t,
        115 as ::core::ffi::c_int as uint8_t,
    ],
    [
        116 as ::core::ffi::c_int as uint8_t,
        122 as ::core::ffi::c_int as uint8_t,
    ],
    [
        123 as ::core::ffi::c_int as uint8_t,
        117 as ::core::ffi::c_int as uint8_t,
    ],
    [
        118 as ::core::ffi::c_int as uint8_t,
        122 as ::core::ffi::c_int as uint8_t,
    ],
    [
        123 as ::core::ffi::c_int as uint8_t,
        119 as ::core::ffi::c_int as uint8_t,
    ],
    [
        120 as ::core::ffi::c_int as uint8_t,
        124 as ::core::ffi::c_int as uint8_t,
    ],
    [
        125 as ::core::ffi::c_int as uint8_t,
        121 as ::core::ffi::c_int as uint8_t,
    ],
    [
        122 as ::core::ffi::c_int as uint8_t,
        126 as ::core::ffi::c_int as uint8_t,
    ],
    [
        127 as ::core::ffi::c_int as uint8_t,
        123 as ::core::ffi::c_int as uint8_t,
    ],
    [
        124 as ::core::ffi::c_int as uint8_t,
        127 as ::core::ffi::c_int as uint8_t,
    ],
    [
        126 as ::core::ffi::c_int as uint8_t,
        125 as ::core::ffi::c_int as uint8_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1708:15"]
pub static mut x264_cabac_renorm_shift: [uint8_t; 64] = [
    6 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "1717:16"]
pub static mut x264_cabac_entropy: [uint16_t; 128] = [
    (0.0273f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.7370f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0288f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.6618f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0303f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.5866f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0320f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.5114f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0337f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.4362f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0355f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.3610f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0375f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.2859f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0395f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.2106f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0416f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.1354f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0439f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (5.0602f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0463f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.9851f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0488f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.9099f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0515f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.8347f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0543f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.7595f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0572f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.6843f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0604f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.6091f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0637f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.5339f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0671f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.4588f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0708f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.3836f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0747f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.3083f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0788f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.2332f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0832f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.1580f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0878f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.0828f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0926f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (4.0076f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.0977f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.9324f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1032f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.8572f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1089f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.7820f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1149f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.7068f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1214f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.6316f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1282f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.5565f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1353f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.4813f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1429f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.4061f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1510f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.3309f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1596f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.2557f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1686f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.1805f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1782f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.1053f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1884f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (3.0301f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.1992f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.9549f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2107f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.8797f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2229f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.8046f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2358f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.7294f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2496f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.6542f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2642f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.5790f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2798f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.5038f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.2964f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.4286f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.3142f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.3534f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.3331f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.2782f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.3532f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.2030f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.3748f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.1278f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.3979f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (2.0527f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.4226f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.9775f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.4491f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.9023f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.4776f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.8271f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.5082f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.7519f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.5412f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.6767f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.5768f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.6015f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.6152f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.5263f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.6568f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.4511f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.7020f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.3759f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.7513f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.3008f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.8050f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.2256f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.8638f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.1504f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (0.9285f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.0752f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.0000f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
    (1.0000f64 * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
        + 0.5f64) as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
#[c2rust::src_loc = "1758:15"]
pub static mut x264_significant_coeff_flag_offset_8x8: [[uint8_t; 64]; 2] = [
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        0,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        0,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1770:15"]
pub static mut x264_last_coeff_flag_offset_8x8: [uint8_t; 63] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "1777:15"]
pub static mut x264_coeff_flag_offset_chroma_422_dc: [uint8_t; 7] = [
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "1778:16"]
pub static mut x264_significant_coeff_flag_offset: [[uint16_t; 16]; 2] = [
    [
        (105 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (105 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (105 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        (105 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as uint16_t,
        (105 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as uint16_t,
        402 as ::core::ffi::c_int as uint16_t,
        (484 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (484 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (484 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        660 as ::core::ffi::c_int as uint16_t,
        (528 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (528 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (528 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        718 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
    ],
    [
        (277 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (277 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (277 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        (277 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as uint16_t,
        (277 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as uint16_t,
        436 as ::core::ffi::c_int as uint16_t,
        (776 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (776 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (776 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        675 as ::core::ffi::c_int as uint16_t,
        (820 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (820 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (820 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        733 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1783:16"]
pub static mut x264_last_coeff_flag_offset: [[uint16_t; 16]; 2] = [
    [
        (166 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (166 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (166 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        (166 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as uint16_t,
        (166 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as uint16_t,
        417 as ::core::ffi::c_int as uint16_t,
        (572 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (572 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (572 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        690 as ::core::ffi::c_int as uint16_t,
        (616 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (616 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (616 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        748 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
    ],
    [
        (338 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (338 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (338 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        (338 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as uint16_t,
        (338 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as uint16_t,
        451 as ::core::ffi::c_int as uint16_t,
        (864 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (864 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (864 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        699 as ::core::ffi::c_int as uint16_t,
        (908 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
        (908 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as uint16_t,
        (908 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as uint16_t,
        757 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
        0 as ::core::ffi::c_int as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "1788:16"]
pub static mut x264_coeff_abs_level_m1_offset: [uint16_t; 16] = [
    (227 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
    (227 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as uint16_t,
    (227 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as uint16_t,
    (227 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as uint16_t,
    (227 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as uint16_t,
    426 as ::core::ffi::c_int as uint16_t,
    (952 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
    (952 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as uint16_t,
    (952 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as uint16_t,
    708 as ::core::ffi::c_int as uint16_t,
    (982 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as uint16_t,
    (982 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as uint16_t,
    (982 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as uint16_t,
    766 as ::core::ffi::c_int as uint16_t,
    0,
    0,
];
#[no_mangle]
#[c2rust::src_loc = "1792:15"]
pub static mut x264_count_cat_m1: [uint8_t; 14] = [
    15 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    63 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
#[c2rust::src_loc = "1799:13"]
pub static mut x264_coeff0_token: [vlc_t; 6] = [
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 1 as uint8_t,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x3 as uint8_t,
            i_size: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0xf as uint8_t,
            i_size: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x3 as uint8_t,
            i_size: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = vlc_t {
            i_bits: 0x1 as uint8_t,
            i_size: 1 as uint8_t,
        };
        init
    },
];
#[no_mangle]
#[c2rust::src_loc = "1810:13"]
pub static mut x264_coeff_token: [[[vlc_t; 4]; 16]; 6] = [
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 2 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 15 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 16 as uint8_t,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 2 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 3 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 14 as uint8_t,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 4 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x10 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x11 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x12 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x13 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x14 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x15 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x16 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x17 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x18 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x19 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1a as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1b as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x1c as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1d as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1e as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1f as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x20 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x21 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x22 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x23 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x24 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x25 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x26 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x27 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x28 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x29 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2a as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2b as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x2c as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2d as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2e as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2f as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x30 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x31 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x32 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x33 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x34 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x35 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x36 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x37 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x38 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x39 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3a as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3b as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x3c as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3d as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3e as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3f as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 1 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x3 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x2 as uint8_t,
                    i_size: 8 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
    ],
    [
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xf as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 2 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0xe as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xd as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 3 as uint8_t,
                };
                init
            },
            vlc_t {
                i_bits: 0,
                i_size: 0,
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xc as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xb as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 5 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0xa as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x1 as uint8_t,
                    i_size: 6 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 9 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x9 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x8 as uint8_t,
                    i_size: 7 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x6 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 10 as uint8_t,
                };
                init
            },
        ],
        [
            {
                let mut init = vlc_t {
                    i_bits: 0x7 as uint8_t,
                    i_size: 13 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x5 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 12 as uint8_t,
                };
                init
            },
            {
                let mut init = vlc_t {
                    i_bits: 0x4 as uint8_t,
                    i_size: 11 as uint8_t,
                };
                init
            },
        ],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
        [vlc_t {
            i_bits: 0,
            i_size: 0,
        }; 4],
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2265:13"]
pub static mut x264_total_zeros: [[vlc_t; 16]; 15] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 7 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 7 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 9 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 9 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 9 as uint8_t,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2435:13"]
pub static mut x264_total_zeros_2x2_dc: [[vlc_t; 4]; 3] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2455:13"]
pub static mut x264_total_zeros_2x4_dc: [[vlc_t; 8]; 7] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2509:13"]
pub static mut x264_run_before_init: [[vlc_t; 16]; 7] = [
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 1 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
    [
        {
            let mut init = vlc_t {
                i_bits: 0x7 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x6 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x5 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x4 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x3 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x2 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 3 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 5 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 6 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 7 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 9 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 10 as uint8_t,
            };
            init
        },
        {
            let mut init = vlc_t {
                i_bits: 0x1 as uint8_t,
                i_size: 11 as uint8_t,
            };
            init
        },
        vlc_t {
            i_bits: 0,
            i_size: 0,
        },
    ],
];
#[no_mangle]
#[c2rust::src_loc = "2570:21"]
pub static mut x264_zero: [uint8_t; 1024] = [
    0 as ::core::ffi::c_int as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn run_static_initializers() {
    x264_dct4_weight_tab = [
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.76777f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.11803f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.70711f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
    ];
    x264_dct8_weight_tab = [
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.0000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.8859f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (1.6000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.9415f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.2651f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.1910f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
    ];
    x264_dct4_weight2_tab = [
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (3.125f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (1.25f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (0.5f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
    ];
    x264_dct8_weight2_tab = [
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 0 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 4 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 2 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 3 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 5 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
        (if 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (1.00000f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (0.78487f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (2.56132f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            (0.88637f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            (1.60040f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else if 1 as ::core::ffi::c_int == 5 as ::core::ffi::c_int {
            (1.41850f64
                * ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as ::core::ffi::c_double
                + 0.5f64) as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint32_t,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
