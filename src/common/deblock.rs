pub mod common_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip_pixel(
        mut x: ::core::ffi::c_int,
    ) -> crate::src::common::common::pixel {
        return (if x & !crate::src::common::common::PIXEL_MAX != 0 {
            -x >> 31 as ::core::ffi::c_int & crate::src::common::common::PIXEL_MAX
        } else {
            x
        }) as crate::src::common::common::pixel;
    }
}
pub mod base_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        };
    }
}
use crate::src::common::deblock::base_h::x264_clip3;
use crate::src::common::deblock::common_h::x264_clip_pixel;
static mut i_alpha_table: [crate::stdlib::uint8_t; 88] = [
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    50 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    56 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    63 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    71 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    80 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    90 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    101 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    113 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    127 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    144 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    162 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    182 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    203 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    226 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    255 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut i_beta_table: [crate::stdlib::uint8_t; 88] = [
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    9 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    12 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    17 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];
static mut i_tc0_table: [[crate::stdlib::int8_t; 4]; 88] = [
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        0 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        1 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        2 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        5 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
        6 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        3 as ::core::ffi::c_int as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
        6 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
        5 as ::core::ffi::c_int as crate::stdlib::int8_t,
        7 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
        5 as ::core::ffi::c_int as crate::stdlib::int8_t,
        8 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        4 as ::core::ffi::c_int as crate::stdlib::int8_t,
        6 as ::core::ffi::c_int as crate::stdlib::int8_t,
        9 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        5 as ::core::ffi::c_int as crate::stdlib::int8_t,
        7 as ::core::ffi::c_int as crate::stdlib::int8_t,
        10 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        6 as ::core::ffi::c_int as crate::stdlib::int8_t,
        8 as ::core::ffi::c_int as crate::stdlib::int8_t,
        11 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        6 as ::core::ffi::c_int as crate::stdlib::int8_t,
        8 as ::core::ffi::c_int as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        7 as ::core::ffi::c_int as crate::stdlib::int8_t,
        10 as ::core::ffi::c_int as crate::stdlib::int8_t,
        14 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        8 as ::core::ffi::c_int as crate::stdlib::int8_t,
        11 as ::core::ffi::c_int as crate::stdlib::int8_t,
        16 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        9 as ::core::ffi::c_int as crate::stdlib::int8_t,
        12 as ::core::ffi::c_int as crate::stdlib::int8_t,
        18 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        10 as ::core::ffi::c_int as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        20 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        11 as ::core::ffi::c_int as crate::stdlib::int8_t,
        15 as ::core::ffi::c_int as crate::stdlib::int8_t,
        23 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t,
        13 as ::core::ffi::c_int as crate::stdlib::int8_t,
        17 as ::core::ffi::c_int as crate::stdlib::int8_t,
        25 as ::core::ffi::c_int as crate::stdlib::int8_t,
    ],
];
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: crate::stdlib::int8_t,
) {
    unsafe {
        let mut p2: ::core::ffi::c_int = *pix
            .offset((-(3 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p1: ::core::ffi::c_int = *pix
            .offset((-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p0: ::core::ffi::c_int = *pix
            .offset((-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q0: ::core::ffi::c_int =
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) as ::core::ffi::c_int;
        let mut q1: ::core::ffi::c_int = *pix
            .offset((1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q2: ::core::ffi::c_int = *pix
            .offset((2 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            let mut tc: ::core::ffi::c_int = tc0 as ::core::ffi::c_int;
            if crate::stdlib::abs(p2 - p0) < beta {
                if tc0 != 0 {
                    *pix.offset(
                        (-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (p1
                        + x264_clip3(
                            (p2 + (p0 + q0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                >> 1 as ::core::ffi::c_int)
                                - p1,
                            -(tc0 as ::core::ffi::c_int),
                            tc0 as ::core::ffi::c_int,
                        )) as crate::src::common::common::pixel;
                }
                tc += 1;
            }
            if crate::stdlib::abs(q2 - q0) < beta {
                if tc0 != 0 {
                    *pix.offset(
                        (1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (q1
                        + x264_clip3(
                            (q2 + (p0 + q0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                >> 1 as ::core::ffi::c_int)
                                - q1,
                            -(tc0 as ::core::ffi::c_int),
                            tc0 as ::core::ffi::c_int,
                        )) as crate::src::common::common::pixel;
                }
                tc += 1;
            }
            let mut delta: ::core::ffi::c_int = x264_clip3(
                (q0 - p0) * 4 as ::core::ffi::c_int + (p1 - q1) + 4 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int,
                -tc,
                tc,
            );
            *pix.offset(
                (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
            ) = x264_clip_pixel(p0 + delta);
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) =
                x264_clip_pixel(q0 - delta);
        }
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            if (*tc0.offset(i as isize) as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                pix = pix.offset(
                    (4 as ::core::ffi::c_int as crate::stdlib::intptr_t * ystride) as isize,
                );
            } else {
                let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while d < 4 as ::core::ffi::c_int {
                    deblock_edge_luma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                    d += 1;
                    pix = pix.offset(ystride as isize);
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn deblock_h_luma_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while d < 8 as ::core::ffi::c_int {
            deblock_edge_luma_c(
                pix,
                1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                alpha,
                beta,
                *tc0.offset((d >> 1 as ::core::ffi::c_int) as isize),
            );
            d += 1;
            pix = pix.offset(stride as isize);
        }
    }
}
unsafe extern "C" fn deblock_v_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_luma_c(
            pix,
            stride,
            1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            alpha,
            beta,
            tc0,
        );
    }
}
unsafe extern "C" fn deblock_h_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_luma_c(
            pix,
            1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
            tc0,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc: crate::stdlib::int8_t,
) {
    unsafe {
        let mut p1: ::core::ffi::c_int = *pix
            .offset((-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p0: ::core::ffi::c_int = *pix
            .offset((-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q0: ::core::ffi::c_int =
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) as ::core::ffi::c_int;
        let mut q1: ::core::ffi::c_int = *pix
            .offset((1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            let mut delta: ::core::ffi::c_int = x264_clip3(
                (q0 - p0) * 4 as ::core::ffi::c_int + (p1 - q1) + 4 as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int,
                -(tc as ::core::ffi::c_int),
                tc as ::core::ffi::c_int,
            );
            *pix.offset(
                (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
            ) = x264_clip_pixel(p0 + delta);
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) =
                x264_clip_pixel(q0 - delta);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut height: ::core::ffi::c_int,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut tc: ::core::ffi::c_int = *tc0.offset(i as isize) as ::core::ffi::c_int;
            if tc <= 0 as ::core::ffi::c_int {
                pix = pix.offset((height as crate::stdlib::intptr_t * ystride) as isize);
            } else {
                let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while d < height {
                    let mut e: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while e < 2 as ::core::ffi::c_int {
                        deblock_edge_chroma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                        e += 1;
                        pix = pix.offset(1);
                    }
                    d += 1;
                    pix = pix.offset(
                        (ystride - 2 as ::core::ffi::c_int as crate::stdlib::intptr_t) as isize,
                    );
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn deblock_h_chroma_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(
            pix,
            1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
            tc0,
        );
    }
}
unsafe extern "C" fn deblock_v_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(
            pix,
            2 as ::core::ffi::c_int,
            stride,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            alpha,
            beta,
            tc0,
        );
    }
}
unsafe extern "C" fn deblock_h_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(
            pix,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
            tc0,
        );
    }
}
unsafe extern "C" fn deblock_h_chroma_422_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(
            pix,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
            tc0,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut p2: ::core::ffi::c_int = *pix
            .offset((-(3 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p1: ::core::ffi::c_int = *pix
            .offset((-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p0: ::core::ffi::c_int = *pix
            .offset((-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q0: ::core::ffi::c_int =
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) as ::core::ffi::c_int;
        let mut q1: ::core::ffi::c_int = *pix
            .offset((1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q2: ::core::ffi::c_int = *pix
            .offset((2 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            if crate::stdlib::abs(p0 - q0)
                < (alpha >> 2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int
            {
                if crate::stdlib::abs(p2 - p0) < beta {
                    let p3: ::core::ffi::c_int = *pix.offset(
                        (-(4 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) as ::core::ffi::c_int;
                    *pix.offset(
                        (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (p2
                        + 2 as ::core::ffi::c_int * p1
                        + 2 as ::core::ffi::c_int * p0
                        + 2 as ::core::ffi::c_int * q0
                        + q1
                        + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                    *pix.offset(
                        (-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (p2 + p1 + p0 + q0 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                    *pix.offset(
                        (-(3 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (2 as ::core::ffi::c_int * p3
                        + 3 as ::core::ffi::c_int * p2
                        + p1
                        + p0
                        + q0
                        + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                } else {
                    *pix.offset(
                        (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (2 as ::core::ffi::c_int * p1 + p0 + q1 + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                }
                if crate::stdlib::abs(q2 - q0) < beta {
                    let q3: ::core::ffi::c_int = *pix.offset(
                        (3 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize,
                    ) as ::core::ffi::c_int;
                    *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) = (p1
                        + 2 as ::core::ffi::c_int * p0
                        + 2 as ::core::ffi::c_int * q0
                        + 2 as ::core::ffi::c_int * q1
                        + q2
                        + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                    *pix.offset(
                        (1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (p0 + q0 + q1 + q2 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                    *pix.offset(
                        (2 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize,
                    ) = (2 as ::core::ffi::c_int * q3
                        + 3 as ::core::ffi::c_int * q2
                        + q1
                        + q0
                        + p0
                        + 4 as ::core::ffi::c_int
                        >> 3 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                } else {
                    *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) =
                        (2 as ::core::ffi::c_int * q1 + q0 + p1 + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                }
            } else {
                *pix.offset(
                    (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
                ) = (2 as ::core::ffi::c_int * p1 + p0 + q1 + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
                *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) =
                    (2 as ::core::ffi::c_int * q1 + q0 + p1 + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while d < 16 as ::core::ffi::c_int {
            deblock_edge_luma_intra_c(pix, xstride, alpha, beta);
            d += 1;
            pix = pix.offset(ystride as isize);
        }
    }
}
unsafe extern "C" fn deblock_h_luma_intra_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while d < 8 as ::core::ffi::c_int {
            deblock_edge_luma_intra_c(
                pix,
                1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                alpha,
                beta,
            );
            d += 1;
            pix = pix.offset(ystride as isize);
        }
    }
}
unsafe extern "C" fn deblock_v_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_luma_intra_c(
            pix,
            stride,
            1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            alpha,
            beta,
        );
    }
}
unsafe extern "C" fn deblock_h_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_luma_intra_c(
            pix,
            1 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut p1: ::core::ffi::c_int = *pix
            .offset((-(2 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut p0: ::core::ffi::c_int = *pix
            .offset((-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        let mut q0: ::core::ffi::c_int =
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) as ::core::ffi::c_int;
        let mut q1: ::core::ffi::c_int = *pix
            .offset((1 as ::core::ffi::c_int as crate::stdlib::intptr_t * xstride) as isize)
            as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            *pix.offset(
                (-(1 as ::core::ffi::c_int) as crate::stdlib::intptr_t * xstride) as isize,
            ) = (2 as ::core::ffi::c_int * p1 + p0 + q1 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
            *pix.offset((0 as crate::stdlib::intptr_t * xstride) as isize) =
                (2 as ::core::ffi::c_int * q1 + q0 + p1 + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while d < height {
            let mut e: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while e < width {
                deblock_edge_chroma_intra_c(pix, xstride, alpha, beta);
                e += 1;
                pix = pix.offset(1);
            }
            d += 1;
            pix =
                pix.offset((ystride - 2 as ::core::ffi::c_int as crate::stdlib::intptr_t) as isize);
        }
    }
}
unsafe extern "C" fn deblock_h_chroma_intra_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(
            pix,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
        );
    }
}
unsafe extern "C" fn deblock_v_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(
            pix,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            stride,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            alpha,
            beta,
        );
    }
}
unsafe extern "C" fn deblock_h_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(
            pix,
            2 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
        );
    }
}
unsafe extern "C" fn deblock_h_chroma_422_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(
            pix,
            2 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int as crate::stdlib::intptr_t,
            stride,
            alpha,
            beta,
        );
    }
}
unsafe extern "C" fn deblock_strength_c(
    mut nnz: *mut crate::stdlib::uint8_t,
    mut ref_0: *mut [crate::stdlib::int8_t; 40],
    mut mv: *mut [[crate::stdlib::int16_t; 2]; 40],
    mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8],
    mut mvy_limit: ::core::ffi::c_int,
    mut bframe: ::core::ffi::c_int,
) {
    unsafe {
        let mut dir: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while dir < 2 as ::core::ffi::c_int {
            let mut s1: ::core::ffi::c_int = if dir != 0 {
                1 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            };
            let mut s2: ::core::ffi::c_int = if dir != 0 {
                8 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            let mut edge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while edge < 4 as ::core::ffi::c_int {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut loc: ::core::ffi::c_int =
                    crate::src::common::base::X264_SCAN8_0 + edge * s2;
                while i < 4 as ::core::ffi::c_int {
                    let mut locn: ::core::ffi::c_int = loc - s2;
                    if *nnz.offset(loc as isize) as ::core::ffi::c_int != 0
                        || *nnz.offset(locn as isize) as ::core::ffi::c_int != 0
                    {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] =
                            2 as crate::stdlib::uint8_t;
                    } else if (*ref_0.offset(0 as ::core::ffi::c_int as isize))[loc as usize]
                        as ::core::ffi::c_int
                        != (*ref_0.offset(0 as ::core::ffi::c_int as isize))[locn as usize]
                            as ::core::ffi::c_int
                        || crate::stdlib::abs(
                            (*mv.offset(0 as ::core::ffi::c_int as isize))[loc as usize]
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - (*mv.offset(0 as ::core::ffi::c_int as isize))[locn as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int,
                        ) >= 4 as ::core::ffi::c_int
                        || crate::stdlib::abs(
                            (*mv.offset(0 as ::core::ffi::c_int as isize))[loc as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - (*mv.offset(0 as ::core::ffi::c_int as isize))[locn as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int,
                        ) >= mvy_limit
                        || bframe != 0
                            && ((*ref_0.offset(1 as ::core::ffi::c_int as isize))[loc as usize]
                                as ::core::ffi::c_int
                                != (*ref_0.offset(1 as ::core::ffi::c_int as isize))[locn as usize]
                                    as ::core::ffi::c_int
                                || crate::stdlib::abs(
                                    (*mv.offset(1 as ::core::ffi::c_int as isize))[loc as usize]
                                        [0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        - (*mv.offset(1 as ::core::ffi::c_int as isize))
                                            [locn as usize]
                                            [0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                ) >= 4 as ::core::ffi::c_int
                                || crate::stdlib::abs(
                                    (*mv.offset(1 as ::core::ffi::c_int as isize))[loc as usize]
                                        [1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        - (*mv.offset(1 as ::core::ffi::c_int as isize))
                                            [locn as usize]
                                            [1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int,
                                ) >= mvy_limit)
                    {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] =
                            1 as crate::stdlib::uint8_t;
                    } else {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] =
                            0 as crate::stdlib::uint8_t;
                    }
                    i += 1;
                    loc += s1;
                }
                edge += 1;
            }
            dir += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge(
    mut _h: *mut crate::src::common::common::x264_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut bS: *mut crate::stdlib::uint8_t,
    mut i_qp: ::core::ffi::c_int,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut pf_inter: crate::src::common::frame::x264_deblock_inter_t,
) {
    unsafe {
        let mut index_a: ::core::ffi::c_int = i_qp + a;
        let mut index_b: ::core::ffi::c_int = i_qp + b;
        let mut alpha: ::core::ffi::c_int =
            (i_alpha_table[(index_a + 24 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int;
        let mut beta: ::core::ffi::c_int =
            (i_beta_table[(index_b + 24 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int;
        let mut tc: [crate::stdlib::int8_t; 4] = [0; 4];
        if (*(bS as *mut crate::src::common::base::x264_union32_t)).i == 0
            || alpha == 0
            || beta == 0
        {
            return;
        }
        tc[0 as ::core::ffi::c_int as usize] = (i_tc0_table
            [(index_a + 24 as ::core::ffi::c_int) as usize]
            [*bS.offset(0 as ::core::ffi::c_int as isize) as usize]
            as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int) << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int)
            + b_chroma) as crate::stdlib::int8_t;
        tc[1 as ::core::ffi::c_int as usize] = (i_tc0_table
            [(index_a + 24 as ::core::ffi::c_int) as usize]
            [*bS.offset(1 as ::core::ffi::c_int as isize) as usize]
            as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int) << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int)
            + b_chroma) as crate::stdlib::int8_t;
        tc[2 as ::core::ffi::c_int as usize] = (i_tc0_table
            [(index_a + 24 as ::core::ffi::c_int) as usize]
            [*bS.offset(2 as ::core::ffi::c_int as isize) as usize]
            as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int) << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int)
            + b_chroma) as crate::stdlib::int8_t;
        tc[3 as ::core::ffi::c_int as usize] = (i_tc0_table
            [(index_a + 24 as ::core::ffi::c_int) as usize]
            [*bS.offset(3 as ::core::ffi::c_int as isize) as usize]
            as ::core::ffi::c_int
            * ((1 as ::core::ffi::c_int) << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int)
            + b_chroma) as crate::stdlib::int8_t;
        pf_inter.expect("non-null function pointer")(
            pix,
            i_stride,
            alpha,
            beta,
            &raw mut tc as *mut crate::stdlib::int8_t,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_intra(
    mut _h: *mut crate::src::common::common::x264_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut _bS: *mut crate::stdlib::uint8_t,
    mut i_qp: ::core::ffi::c_int,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut _b_chroma: ::core::ffi::c_int,
    mut pf_intra: crate::src::common::frame::x264_deblock_intra_t,
) {
    unsafe {
        let mut index_a: ::core::ffi::c_int = i_qp + a;
        let mut index_b: ::core::ffi::c_int = i_qp + b;
        let mut alpha: ::core::ffi::c_int =
            (i_alpha_table[(index_a + 24 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int;
        let mut beta: ::core::ffi::c_int =
            (i_beta_table[(index_b + 24 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
                << crate::internal::BIT_DEPTH - 8 as ::core::ffi::c_int;
        if alpha == 0 || beta == 0 {
            return;
        }
        pf_intra.expect("non-null function pointer")(pix, i_stride, alpha, beta);
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours_deblock(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut deblock_on_slice_edges: ::core::ffi::c_int =
            ((*h).sh.i_disable_deblocking_filter_idc != 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        (*h).mb.i_neighbour = 0 as ::core::ffi::c_uint;
        (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.b_interlaced = ((*h).param.b_interlaced != 0
            && *(*h).mb.field.offset((*h).mb.i_mb_xy as isize) as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        (*h).mb.i_mb_top_y = mb_y - ((1 as ::core::ffi::c_int) << (*h).mb.b_interlaced);
        (*h).mb.i_mb_top_xy = mb_x + (*h).mb.i_mb_stride * (*h).mb.i_mb_top_y;
        (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int;
        (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize];
        if (*h).sh.b_mbaff != 0 {
            if mb_y & 1 as ::core::ffi::c_int != 0 {
                if mb_x != 0
                    && *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] -= (*h).mb.i_mb_stride;
                }
            } else {
                if (*h).mb.i_mb_top_xy >= 0 as ::core::ffi::c_int
                    && (*h).mb.b_interlaced != 0
                    && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0
                {
                    (*h).mb.i_mb_top_xy += (*h).mb.i_mb_stride;
                    (*h).mb.i_mb_top_y += 1;
                }
                if mb_x != 0
                    && *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
                }
            }
        }
        if mb_x > 0 as ::core::ffi::c_int
            && (deblock_on_slice_edges != 0
                || *(*h)
                    .mb
                    .slice_table
                    .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                    == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
        {
            (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
        if mb_y > (*h).mb.b_interlaced
            && (deblock_on_slice_edges != 0
                || *(*h).mb.slice_table.offset((*h).mb.i_mb_top_xy as isize)
                    == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
        {
            (*h).mb.i_neighbour |=
                crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_deblock_row(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_interlaced: ::core::ffi::c_int = (*h).sh.b_mbaff;
        let mut a: ::core::ffi::c_int =
            (*h).sh.i_alpha_c0_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut b: ::core::ffi::c_int =
            (*h).sh.i_beta_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut qp_thresh: ::core::ffi::c_int = 15 as ::core::ffi::c_int
            - (if a < b { a } else { b })
            - (if 0 as ::core::ffi::c_int
                > (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            {
                0 as ::core::ffi::c_int
            } else {
                (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            });
        let mut stridey: ::core::ffi::c_int =
            (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize];
        let mut strideuv: ::core::ffi::c_int =
            (*(*h).fdec).i_stride[1 as ::core::ffi::c_int as usize];
        let mut chroma_format: ::core::ffi::c_int =
            crate::src::common::base::CHROMA_444 as ::core::ffi::c_int;
        let mut chroma444: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
            as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut chroma_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        let mut uvdiff: crate::stdlib::intptr_t = if chroma444 != 0 {
            (*(*h).fdec).plane[2 as ::core::ffi::c_int as usize]
                .offset_from((*(*h).fdec).plane[1 as ::core::ffi::c_int as usize])
                as crate::stdlib::intptr_t
        } else {
            1 as ::core::ffi::c_int as crate::stdlib::intptr_t
        };
        let mut mb_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while mb_x < (*h).mb.i_mb_width {
            crate::src::common::macroblock::x264_8_prefetch_fenc(
                h as *mut crate::src::common::common::x264_t,
                (*h).fdec as *mut crate::src::common::frame::x264_frame,
                mb_x,
                mb_y,
            );
            macroblock_cache_load_neighbours_deblock(h, mb_x, mb_y);
            let mut mb_xy: ::core::ffi::c_int = (*h).mb.i_mb_xy;
            let mut transform_8x8: ::core::ffi::c_int =
                *(*h).mb.mb_transform_size.offset(mb_xy as isize) as ::core::ffi::c_int;
            let mut intra_cur: ::core::ffi::c_int = (*(*h).mb.type_0.offset(mb_xy as isize)
                as ::core::ffi::c_int
                == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            let mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8] =
                &raw mut *(*(&raw mut (*h).deblock_strength
                    as *mut *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2])
                    .offset((mb_y & 1 as ::core::ffi::c_int) as isize))
                .offset(
                    (if (*h).param.b_sliced_threads != 0 {
                        mb_xy
                    } else {
                        mb_x
                    }) as isize,
                ) as *mut [[crate::stdlib::uint8_t; 4]; 8];
            let mut pixy: *mut crate::src::common::common::pixel = (*(*h).fdec).plane
                [0 as ::core::ffi::c_int as usize]
                .offset((16 as ::core::ffi::c_int * mb_y * stridey) as isize)
                .offset((16 as ::core::ffi::c_int * mb_x) as isize);
            let mut pixuv: *mut crate::src::common::common::pixel =
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    (*(*h).fdec).plane[1 as ::core::ffi::c_int as usize]
                        .offset((chroma_height * mb_y * strideuv) as isize)
                        .offset((16 as ::core::ffi::c_int * mb_x) as isize)
                } else {
                    ::core::ptr::null_mut::<crate::src::common::common::pixel>()
                };
            if mb_y & (*h).mb.b_interlaced != 0 {
                pixy = pixy.offset(-((15 as ::core::ffi::c_int * stridey) as isize));
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    pixuv = pixuv
                        .offset(-(((chroma_height - 1 as ::core::ffi::c_int) * strideuv) as isize));
                }
            }
            let mut stride2y: ::core::ffi::c_int = stridey << (*h).mb.b_interlaced;
            let mut stride2uv: ::core::ffi::c_int = strideuv << (*h).mb.b_interlaced;
            let mut qp: ::core::ffi::c_int =
                *(*h).mb.qp.offset(mb_xy as isize) as ::core::ffi::c_int;
            let mut qpc: ::core::ffi::c_int =
                *(*h).chroma_qp_table.offset(qp as isize) as ::core::ffi::c_int;
            let mut first_edge_only: ::core::ffi::c_int =
                (*(*h).mb.partition.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                    && *(*h).mb.cbp.offset(mb_xy as isize) == 0
                    && intra_cur == 0
                    || qp <= qp_thresh) as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                if b_interlaced != 0
                    && *(*h)
                        .mb
                        .field
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    let mut luma_qp: [::core::ffi::c_int; 2] = [0; 2];
                    let mut chroma_qp: [::core::ffi::c_int; 2] = [0; 2];
                    let mut left_qp: [::core::ffi::c_int; 2] = [0; 2];
                    let mut luma_deblock: crate::src::common::frame::x264_deblock_inter_t =
                        (*h).loopf.deblock_luma_mbaff;
                    let mut chroma_deblock: crate::src::common::frame::x264_deblock_inter_t =
                        (*h).loopf.deblock_chroma_mbaff;
                    let mut luma_intra_deblock: crate::src::common::frame::x264_deblock_intra_t =
                        (*h).loopf.deblock_luma_intra_mbaff;
                    let mut chroma_intra_deblock: crate::src::common::frame::x264_deblock_intra_t =
                        (*h).loopf.deblock_chroma_intra_mbaff;
                    let mut c: ::core::ffi::c_int = if chroma444 != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    left_qp[0 as ::core::ffi::c_int as usize] = *(*h)
                        .mb
                        .qp
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int;
                    luma_qp[0 as ::core::ffi::c_int as usize] =
                        qp + left_qp[0 as ::core::ffi::c_int as usize] + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int;
                    chroma_qp[0 as ::core::ffi::c_int as usize] = qpc
                        + *(*h)
                            .chroma_qp_table
                            .offset(left_qp[0 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int;
                    if intra_cur != 0
                        || (*(*h)
                            .mb
                            .type_0
                            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        deblock_edge_intra(
                            h,
                            pixy,
                            (2 as ::core::ffi::c_int * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[0 as ::core::ffi::c_int as usize],
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            luma_intra_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv,
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[0 as ::core::ffi::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff as isize),
                                    (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[0 as ::core::ffi::c_int as usize],
                                    a,
                                    b,
                                    c,
                                    chroma_intra_deblock,
                                );
                            }
                        }
                    } else {
                        deblock_edge(
                            h,
                            pixy,
                            (2 as ::core::ffi::c_int * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[0 as ::core::ffi::c_int as usize],
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            luma_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv,
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[0 as ::core::ffi::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff as isize),
                                    (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[0 as ::core::ffi::c_int as usize],
                                    a,
                                    b,
                                    c,
                                    chroma_deblock,
                                );
                            }
                        }
                    }
                    let mut offy: ::core::ffi::c_int = if (*h).mb.b_interlaced != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut offuv: ::core::ffi::c_int = if (*h).mb.b_interlaced != 0 {
                        4 as ::core::ffi::c_int
                            - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    left_qp[1 as ::core::ffi::c_int as usize] = *(*h)
                        .mb
                        .qp
                        .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int;
                    luma_qp[1 as ::core::ffi::c_int as usize] =
                        qp + left_qp[1 as ::core::ffi::c_int as usize] + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int;
                    chroma_qp[1 as ::core::ffi::c_int as usize] = qpc
                        + *(*h)
                            .chroma_qp_table
                            .offset(left_qp[1 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int;
                    if intra_cur != 0
                        || (*(*h)
                            .mb
                            .type_0
                            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(
                                (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize,
                            ) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        deblock_edge_intra(
                            h,
                            pixy.offset((stridey << offy) as isize),
                            (2 as ::core::ffi::c_int * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[1 as ::core::ffi::c_int as usize],
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            luma_intra_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv.offset((strideuv << offuv) as isize),
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[1 as ::core::ffi::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge_intra(
                                    h,
                                    pixuv
                                        .offset(uvdiff as isize)
                                        .offset((strideuv << offuv) as isize),
                                    (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[1 as ::core::ffi::c_int as usize],
                                    a,
                                    b,
                                    c,
                                    chroma_intra_deblock,
                                );
                            }
                        }
                    } else {
                        deblock_edge(
                            h,
                            pixy.offset((stridey << offy) as isize),
                            (2 as ::core::ffi::c_int * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(4 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[1 as ::core::ffi::c_int as usize],
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            luma_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((strideuv << offuv) as isize),
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[1 as ::core::ffi::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge(
                                    h,
                                    pixuv
                                        .offset(uvdiff as isize)
                                        .offset((strideuv << offuv) as isize),
                                    (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[1 as ::core::ffi::c_int as usize],
                                    a,
                                    b,
                                    c,
                                    chroma_deblock,
                                );
                            }
                        }
                    }
                } else {
                    let mut qpl: ::core::ffi::c_int = *(*h)
                        .mb
                        .qp
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                    let mut qp_left: ::core::ffi::c_int =
                        qp + qpl + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
                    let mut qpc_left: ::core::ffi::c_int = qpc
                        + *(*h).chroma_qp_table.offset(qpl as isize) as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int;
                    let mut intra_left: ::core::ffi::c_int = (*(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || *(*h)
                            .mb
                            .type_0
                            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || *(*h)
                            .mb
                            .type_0
                            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || *(*h)
                            .mb
                            .type_0
                            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    let mut intra_deblock: ::core::ffi::c_int =
                        (intra_cur != 0 || intra_left != 0) as ::core::ffi::c_int;
                    if !(*(*h).fdec).mb_info.is_null()
                        && (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i
                            != 0
                    {
                        let ref mut c2rust_fresh0 =
                            *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                        *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int
                            | 0xff as ::core::ffi::c_int
                                * (*(*(*h).fdec).mb_info.offset(mb_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                        let ref mut c2rust_fresh1 = *(*(*h).fdec).effective_qp.offset(
                            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize,
                        );
                        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                            | 0xff as ::core::ffi::c_int
                                * (*(*(*h).fdec).mb_info.offset(
                                    (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize,
                                ) as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                    }
                    if intra_deblock != 0 {
                        if 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            || transform_8x8 == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixy.offset(
                                    (4 as ::core::ffi::c_int
                                        * 0 as ::core::ffi::c_int
                                        * (if 0 as ::core::ffi::c_int != 0 {
                                            stride2y
                                        } else {
                                            1 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_left,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma_intra[0 as ::core::ffi::c_int as usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma_intra[0 as ::core::ffi::c_int as usize],
                                );
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff as isize).offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma_intra[0 as ::core::ffi::c_int as usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                2 as ::core::ffi::c_int * stride2uv
                                            } else {
                                                4 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    1 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_chroma_intra
                                        [0 as ::core::ffi::c_int as usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (0 as ::core::ffi::c_int != 0
                                || 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as ::core::ffi::c_int
                                        * (if 0 as ::core::ffi::c_int != 0 {
                                            4 as ::core::ffi::c_int * stride2uv
                                        } else {
                                            4 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_left,
                                a,
                                b,
                                1 as ::core::ffi::c_int,
                                (*h).loopf.deblock_chroma_intra[0 as ::core::ffi::c_int as usize],
                            );
                        }
                    } else {
                        if 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            || transform_8x8 == 0
                        {
                            deblock_edge(
                                h,
                                pixy.offset(
                                    (4 as ::core::ffi::c_int
                                        * 0 as ::core::ffi::c_int
                                        * (if 0 as ::core::ffi::c_int != 0 {
                                            stride2y
                                        } else {
                                            1 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_left,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                                );
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff as isize).offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (0 as ::core::ffi::c_int
                                            * (if 0 as ::core::ffi::c_int != 0 {
                                                2 as ::core::ffi::c_int * stride2uv
                                            } else {
                                                4 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    1 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (0 as ::core::ffi::c_int != 0
                                || 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as ::core::ffi::c_int
                                        * (if 0 as ::core::ffi::c_int != 0 {
                                            4 as ::core::ffi::c_int * stride2uv
                                        } else {
                                            4 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_left,
                                a,
                                b,
                                1 as ::core::ffi::c_int,
                                (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                            );
                        }
                    }
                }
            }
            if first_edge_only == 0 {
                if 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 1 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (1 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0 as ::core::ffi::c_int != 0
                        || 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                    );
                }
                if 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (2 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0 as ::core::ffi::c_int != 0
                        || 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                    );
                }
                if 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 3 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (3 as ::core::ffi::c_int
                                    * (if 0 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0 as ::core::ffi::c_int != 0
                        || 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as ::core::ffi::c_int
                                * (if 0 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[0 as ::core::ffi::c_int as usize],
                    );
                }
            }
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                if b_interlaced != 0
                    && mb_y & 1 as ::core::ffi::c_int == 0
                    && (*h).mb.b_interlaced == 0
                    && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                        != 0
                {
                    let mut mbn_xy: ::core::ffi::c_int =
                        mb_xy - 2 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < 2 as ::core::ffi::c_int {
                        let mut qpt: ::core::ffi::c_int =
                            *(*h).mb.qp.offset(mbn_xy as isize) as ::core::ffi::c_int;
                        let mut qp_top: ::core::ffi::c_int =
                            qp + qpt + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
                        let mut qpc_top: ::core::ffi::c_int = qpc
                            + *(*h).chroma_qp_table.offset(qpt as isize) as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int;
                        let mut intra_top: ::core::ffi::c_int =
                            (*(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                                || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                    == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                                || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                    == crate::src::common::macroblock::I_16x16
                                        as ::core::ffi::c_int
                                || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                    == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                        if intra_cur != 0 || intra_top != 0 {
                            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset((4 as ::core::ffi::c_int * j) as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0x3030303 as crate::stdlib::uint32_t;
                        }
                        deblock_edge(
                            h,
                            pixy.offset((j * stridey) as isize),
                            (2 as ::core::ffi::c_int * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset((4 as ::core::ffi::c_int * j) as isize)
                                as *mut crate::stdlib::uint8_t,
                            qp_top,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((j * strideuv) as isize),
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4 as ::core::ffi::c_int * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                            );
                            deblock_edge(
                                h,
                                pixuv
                                    .offset(uvdiff as isize)
                                    .offset((j * strideuv) as isize),
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4 as ::core::ffi::c_int * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                            );
                        } else if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((j * strideuv) as isize),
                                (2 as ::core::ffi::c_int * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4 as ::core::ffi::c_int * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                1 as ::core::ffi::c_int,
                                (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                            );
                        }
                        j += 1;
                        mbn_xy += (*h).mb.i_mb_stride;
                    }
                } else {
                    let mut qpt_0: ::core::ffi::c_int =
                        *(*h).mb.qp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
                    let mut qp_top_0: ::core::ffi::c_int =
                        qp + qpt_0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
                    let mut qpc_top_0: ::core::ffi::c_int = qpc
                        + *(*h).chroma_qp_table.offset(qpt_0 as isize) as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int;
                    let mut intra_top_0: ::core::ffi::c_int =
                        (*(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    let mut intra_deblock_0: ::core::ffi::c_int =
                        (intra_cur != 0 || intra_top_0 != 0) as ::core::ffi::c_int;
                    if !(*(*h).fdec).mb_info.is_null()
                        && (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union32_t))
                            .i
                            != 0
                    {
                        let ref mut c2rust_fresh2 =
                            *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                        *c2rust_fresh2 = (*c2rust_fresh2 as ::core::ffi::c_int
                            | 0xff as ::core::ffi::c_int
                                * (*(*(*h).fdec).mb_info.offset(mb_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                        let ref mut c2rust_fresh3 = *(*(*h).fdec)
                            .effective_qp
                            .offset((*h).mb.i_mb_top_xy as isize);
                        *c2rust_fresh3 = (*c2rust_fresh3 as ::core::ffi::c_int
                            | 0xff as ::core::ffi::c_int
                                * (*(*(*h).fdec).mb_info.offset((*h).mb.i_mb_top_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                    }
                    if (b_interlaced == 0
                        || (*h).mb.b_interlaced == 0
                            && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0)
                        && intra_deblock_0 != 0
                    {
                        if 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            || transform_8x8 == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixy.offset(
                                    (4 as ::core::ffi::c_int
                                        * 0 as ::core::ffi::c_int
                                        * (if 1 as ::core::ffi::c_int != 0 {
                                            stride2y
                                        } else {
                                            1 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_top_0,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma_intra[1 as ::core::ffi::c_int as usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma_intra[1 as ::core::ffi::c_int as usize],
                                );
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff as isize).offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma_intra[1 as ::core::ffi::c_int as usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                2 as ::core::ffi::c_int * stride2uv
                                            } else {
                                                4 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    1 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_chroma_intra
                                        [1 as ::core::ffi::c_int as usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (1 as ::core::ffi::c_int != 0
                                || 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as ::core::ffi::c_int
                                        * (if 1 as ::core::ffi::c_int != 0 {
                                            4 as ::core::ffi::c_int * stride2uv
                                        } else {
                                            4 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top_0,
                                a,
                                b,
                                1 as ::core::ffi::c_int,
                                (*h).loopf.deblock_chroma_intra[1 as ::core::ffi::c_int as usize],
                            );
                        }
                    } else {
                        if intra_deblock_0 != 0 {
                            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0x3030303 as crate::stdlib::uint32_t;
                        }
                        if 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            || transform_8x8 == 0
                        {
                            deblock_edge(
                                h,
                                pixy.offset(
                                    (4 as ::core::ffi::c_int
                                        * 0 as ::core::ffi::c_int
                                        * (if 1 as ::core::ffi::c_int != 0 {
                                            stride2y
                                        } else {
                                            1 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_top_0,
                                a,
                                b,
                                0 as ::core::ffi::c_int,
                                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                                );
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff as isize).offset(
                                        (4 as ::core::ffi::c_int
                                            * 0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                stride2uv
                                            } else {
                                                1 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (0 as ::core::ffi::c_int
                                            * (if 1 as ::core::ffi::c_int != 0 {
                                                2 as ::core::ffi::c_int * stride2uv
                                            } else {
                                                4 as ::core::ffi::c_int
                                            })) as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    1 as ::core::ffi::c_int,
                                    (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (1 as ::core::ffi::c_int != 0
                                || 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as ::core::ffi::c_int
                                        * (if 1 as ::core::ffi::c_int != 0 {
                                            4 as ::core::ffi::c_int * stride2uv
                                        } else {
                                            4 as ::core::ffi::c_int
                                        })) as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top_0,
                                a,
                                b,
                                1 as ::core::ffi::c_int,
                                (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                            );
                        }
                    }
                }
            }
            if first_edge_only == 0 {
                if 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 1 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (1 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1 as ::core::ffi::c_int != 0
                        || 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                    );
                }
                if 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (2 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1 as ::core::ffi::c_int != 0
                        || 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                    );
                }
                if 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4 as ::core::ffi::c_int
                                * 3 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    stride2y
                                } else {
                                    1 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0 as ::core::ffi::c_int,
                        (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff as isize).offset(
                                (4 as ::core::ffi::c_int
                                    * 3 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        stride2uv
                                    } else {
                                        1 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0 as ::core::ffi::c_int,
                            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (3 as ::core::ffi::c_int
                                    * (if 1 as ::core::ffi::c_int != 0 {
                                        2 as ::core::ffi::c_int * stride2uv
                                    } else {
                                        4 as ::core::ffi::c_int
                                    })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1 as ::core::ffi::c_int,
                            (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1 as ::core::ffi::c_int != 0
                        || 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as ::core::ffi::c_int
                                * (if 1 as ::core::ffi::c_int != 0 {
                                    4 as ::core::ffi::c_int * stride2uv
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1 as ::core::ffi::c_int,
                        (*h).loopf.deblock_chroma[1 as ::core::ffi::c_int as usize],
                    );
                }
            }
            mb_x += (!b_interlaced | mb_y) & 1 as ::core::ffi::c_int;
            mb_y ^= b_interlaced;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_deblock(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut a: ::core::ffi::c_int =
            (*h).sh.i_alpha_c0_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut b: ::core::ffi::c_int =
            (*h).sh.i_beta_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut qp_thresh: ::core::ffi::c_int = 15 as ::core::ffi::c_int
            - (if a < b { a } else { b })
            - (if 0 as ::core::ffi::c_int
                > (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            {
                0 as ::core::ffi::c_int
            } else {
                (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            });
        let mut intra_cur: ::core::ffi::c_int = ((*h).mb.i_type
            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut qp: ::core::ffi::c_int = (*h).mb.i_qp;
        let mut qpc: ::core::ffi::c_int = (*h).mb.i_chroma_qp;
        if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma == 0
            && intra_cur == 0
            || qp <= qp_thresh
        {
            return;
        }
        let mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
        if intra_cur != 0 {
            (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303 as crate::stdlib::uint32_t;
            (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303 as crate::stdlib::uint64_t;
            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303 as crate::stdlib::uint32_t;
            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303 as crate::stdlib::uint64_t;
        } else {
            (*h).loopf
                .deblock_strength
                .expect("non-null function pointer")(
                &raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t,
                &raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40],
                &raw mut (*h).mb.cache.mv as *mut [[crate::stdlib::int16_t; 2]; 40],
                bs as *mut [[crate::stdlib::uint8_t; 4]; 8],
                4 as ::core::ffi::c_int >> (*h).mb.b_interlaced,
                ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
            );
        }
        let mut transform_8x8: ::core::ffi::c_int = (*h).mb.b_transform_8x8;
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        * (if 0 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            * (if 0 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            * (if 0 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                );
            }
        }
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                (4 as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    * (if 0 as ::core::ffi::c_int != 0 {
                        crate::src::common::common::FDEC_STRIDE
                    } else {
                        1 as ::core::ffi::c_int
                    })) as isize,
            ),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t,
            qp,
            a,
            b,
            0 as ::core::ffi::c_int,
            (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * (if 0 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * (if 0 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
            );
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        * (if 0 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(3 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int
                            * (if 0 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int
                            * (if 0 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[0 as ::core::ffi::c_int as usize],
                );
            }
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        * (if 1 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            * (if 1 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            * (if 1 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                );
            }
        }
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                (4 as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    * (if 1 as ::core::ffi::c_int != 0 {
                        crate::src::common::common::FDEC_STRIDE
                    } else {
                        1 as ::core::ffi::c_int
                    })) as isize,
            ),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t,
            qp,
            a,
            b,
            0 as ::core::ffi::c_int,
            (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * (if 1 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        * (if 1 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
            );
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                    (4 as ::core::ffi::c_int
                        * 3 as ::core::ffi::c_int
                        * (if 1 as ::core::ffi::c_int != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1 as ::core::ffi::c_int
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(3 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0 as ::core::ffi::c_int,
                (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int
                            * (if 1 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                        (4 as ::core::ffi::c_int
                            * 3 as ::core::ffi::c_int
                            * (if 1 as ::core::ffi::c_int != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1 as ::core::ffi::c_int
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0 as ::core::ffi::c_int,
                    (*h).loopf.deblock_luma[1 as ::core::ffi::c_int as usize],
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_deblock_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::frame::x264_deblock_function_t,
    mut _b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        (*pf).deblock_luma[1 as ::core::ffi::c_int as usize] = Some(
            deblock_v_luma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_luma[0 as ::core::ffi::c_int as usize] = Some(
            deblock_h_luma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_chroma[1 as ::core::ffi::c_int as usize] = Some(
            deblock_v_chroma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_h_chroma_420 = Some(
            deblock_h_chroma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        ) as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_h_chroma_422 = Some(
            deblock_h_chroma_422_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        ) as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_luma_intra[1 as ::core::ffi::c_int as usize] = Some(
            deblock_v_luma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_luma_intra[0 as ::core::ffi::c_int as usize] = Some(
            deblock_h_luma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_chroma_intra[1 as ::core::ffi::c_int as usize] = Some(
            deblock_v_chroma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_h_chroma_420_intra = Some(
            deblock_h_chroma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_h_chroma_422_intra = Some(
            deblock_h_chroma_422_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_luma_mbaff = Some(
            deblock_h_luma_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        ) as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_chroma_420_mbaff = Some(
            deblock_h_chroma_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_inter_t;
        (*pf).deblock_luma_intra_mbaff = Some(
            deblock_h_luma_intra_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_chroma_420_intra_mbaff = Some(
            deblock_h_chroma_intra_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::frame::x264_deblock_intra_t;
        (*pf).deblock_strength = Some(
            deblock_strength_c
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut [crate::stdlib::int8_t; 40],
                    *mut [[crate::stdlib::int16_t; 2]; 40],
                    *mut [[crate::stdlib::uint8_t; 4]; 8],
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut [crate::stdlib::int8_t; 40],
                    *mut [[crate::stdlib::int16_t; 2]; 40],
                    *mut [[crate::stdlib::uint8_t; 4]; 8],
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).deblock_chroma_422_mbaff = (*pf).deblock_h_chroma_420;
        (*pf).deblock_chroma_422_intra_mbaff = (*pf).deblock_h_chroma_420_intra;
    }
}
