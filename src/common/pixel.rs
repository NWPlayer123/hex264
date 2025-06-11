#[cfg(feature = "high_bit_depth")]
type Pixel = u16;
#[cfg(not(feature = "high_bit_depth"))]
type Pixel = u8;

type PixelCmpFn = fn(&[Pixel], usize, &[Pixel], usize) -> usize;
type PixelCmpX3Fn = fn(&[Pixel], &[Pixel], &[Pixel], &[Pixel], usize, &mut [usize; 3]);
type PixelCmpX4Fn = fn(&[Pixel], &[Pixel], &[Pixel], &[Pixel], &[Pixel], usize, &mut [usize; 4]);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelSize {
    Pixel16x16 = 0,
    Pixel16x8 = 1,
    Pixel8x16 = 2,
    Pixel8x8 = 3,
    Pixel8x4 = 4,
    Pixel4x8 = 5,
    Pixel4x4 = 6,

    // Subsampled chroma only
    Pixel4x16 = 7, // 4:2:2
    Pixel4x2 = 8,
    Pixel2x8 = 9, // 4:2:2
    Pixel2x4 = 10,
    Pixel2x2 = 11,
}

const PIXEL_SIZES: [(u8, u8); 12] =
    [(16, 16), (16, 8), (8, 16), (8, 8), (8, 4), (4, 8), (4, 4), (4, 16), (4, 2), (2, 8), (2, 4), (2, 2)];

const SIZE_TO_PIXEL: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, PixelSize::Pixel4x4 as u8, PixelSize::Pixel8x4 as u8, 0, 0],
    [0, PixelSize::Pixel4x8 as u8, PixelSize::Pixel8x8 as u8, 0, PixelSize::Pixel16x8 as u8],
    [0, 0, 0, 0, 0],
    [0, 0, PixelSize::Pixel8x16 as u8, 0, PixelSize::Pixel16x16 as u8],
];

const LUMA_TO_CHROMA_PIXEL: [[u8; 7]; 4] = [
    [0, 0, 0, 0, 0, 0, 0], // unused
    [
        PixelSize::Pixel8x8 as u8,
        PixelSize::Pixel8x4 as u8,
        PixelSize::Pixel4x8 as u8,
        PixelSize::Pixel4x4 as u8,
        PixelSize::Pixel4x2 as u8,
        PixelSize::Pixel2x4 as u8,
        PixelSize::Pixel2x2 as u8,
    ], // 4:2:0
    [
        PixelSize::Pixel8x16 as u8,
        PixelSize::Pixel8x8 as u8,
        PixelSize::Pixel4x16 as u8,
        PixelSize::Pixel4x8 as u8,
        PixelSize::Pixel4x4 as u8,
        PixelSize::Pixel2x8 as u8,
        PixelSize::Pixel2x4 as u8,
    ], // 4:2:2
    [
        PixelSize::Pixel16x16 as u8,
        PixelSize::Pixel16x8 as u8,
        PixelSize::Pixel8x16 as u8,
        PixelSize::Pixel8x8 as u8,
        PixelSize::Pixel8x4 as u8,
        PixelSize::Pixel4x8 as u8,
        PixelSize::Pixel4x4 as u8,
    ], // 4:4:4
];

struct PixelFunctions {
    pub sad: [PixelCmpFn; 8],
    pub ssd: [PixelCmpFn; 8],
}

impl Default for PixelFunctions {
    fn default() -> Self {
        Self {
            sad: [
                pixel_sad_16x16,
                pixel_sad_16x8,
                pixel_sad_8x16,
                pixel_sad_8x8,
                pixel_sad_8x4,
                pixel_sad_4x8,
                pixel_sad_4x4,
                pixel_sad_4x16,
            ],
            ssd: [
                pixel_ssd_16x16,
                pixel_ssd_16x8,
                pixel_ssd_8x16,
                pixel_ssd_8x8,
                pixel_ssd_8x4,
                pixel_ssd_4x8,
                pixel_ssd_4x4,
                pixel_ssd_4x16,
            ],
        }
    }
}

macro_rules! pixel_sad {
    ($name:ident, $width:expr, $height:expr) => {
        fn $name(pix1: &[Pixel], stride1: usize, pix2: &[Pixel], stride2: usize) -> usize {
            let mut sum = 0;
            for y in 0..$height {
                for x in 0..$width {
                    sum += Pixel::abs_diff(pix1[x + (y * stride1)], pix2[x + (y * stride2)]) as usize;
                }
            }
            sum
        }
    };
}

pixel_sad!(pixel_sad_16x16, 16, 16);
pixel_sad!(pixel_sad_16x8, 16, 8);
pixel_sad!(pixel_sad_8x16, 8, 16);
pixel_sad!(pixel_sad_8x8, 8, 8);
pixel_sad!(pixel_sad_8x4, 8, 4);
pixel_sad!(pixel_sad_4x16, 4, 16);
pixel_sad!(pixel_sad_4x8, 4, 8);
pixel_sad!(pixel_sad_4x4, 4, 4);

macro_rules! pixel_ssd {
    ($name:ident, $width:expr, $height:expr) => {
        fn $name(pix1: &[Pixel], stride1: usize, pix2: &[Pixel], stride2: usize) -> usize {
            let mut sum = 0;
            for y in 0..$height {
                for x in 0..$width {
                    let diff = pix1[x + (y * stride1)] as i32 - pix2[x + (y * stride2)] as i32;
                    sum += (diff * diff) as usize;
                }
            }
            sum
        }
    };
}

pixel_ssd!(pixel_ssd_16x16, 16, 16);
pixel_ssd!(pixel_ssd_16x8, 16, 8);
pixel_ssd!(pixel_ssd_8x16, 8, 16);
pixel_ssd!(pixel_ssd_8x8, 8, 8);
pixel_ssd!(pixel_ssd_8x4, 8, 4);
pixel_ssd!(pixel_ssd_4x16, 4, 16);
pixel_ssd!(pixel_ssd_4x8, 4, 8);
pixel_ssd!(pixel_ssd_4x4, 4, 4);

fn pixel_ssd_wxh(
    pf: &PixelFunctions, pix1: &[Pixel], stride1: usize, pix2: &[Pixel], stride2: usize, width: u32,
    height: u32,
) -> u64 {
    let mut ssd = 0;
    let align = ((pix1.as_ptr() as usize | pix2.as_ptr() as usize | stride1 | stride2) & 15) == 0;

    let mut y = 0;
    while y < height.saturating_sub(15) {
        let mut x = 0;
        if align {
            while x < width.saturating_sub(15) {
                let off1 = (y as usize) * stride1 + (x as usize);
                let off2 = (y as usize) * stride2 + (x as usize);
                ssd += pf.ssd[PixelSize::Pixel16x16 as usize](&pix1[off1..], stride1, &pix2[off2..], stride2)
                    as u64;

                x += 16;
            }
        }

        while x < width.saturating_sub(7) {
            let off1 = (y as usize) * stride1 + (x as usize);
            let off2 = (y as usize) * stride2 + (x as usize);
            ssd +=
                pf.ssd[PixelSize::Pixel8x16 as usize](&pix1[off1..], stride1, &pix2[off2..], stride2) as u64;

            x += 8;
        }
        y += 16;
    }

    if y < height.saturating_sub(7) {
        for x in (0..width.saturating_sub(7)).step_by(8) {
            let off1 = (y as usize) * stride1 + (x as usize);
            let off2 = (y as usize) * stride2 + (x as usize);
            ssd +=
                pf.ssd[PixelSize::Pixel8x8 as usize](&pix1[off1..], stride1, &pix2[off2..], stride2) as u64;
        }
    }

    if width & 7 != 0 {
        for y in 0..(height & !7) {
            for x in (width & !7)..width {
                let idx1 = (y as usize) * stride1 + (x as usize);
                let idx2 = (y as usize) * stride2 + (x as usize);
                let diff = pix1[idx1] as i32 - pix2[idx2] as i32;
                ssd += (diff * diff) as u64;
            }
        }
    }

    if height & 7 != 0 {
        for y in (height & !7)..height {
            for x in 0..width {
                let idx1 = (y as usize) * stride1 + (x as usize);
                let idx2 = (y as usize) * stride2 + (x as usize);
                let diff = pix1[idx1] as i32 - pix2[idx2] as i32;
                ssd += (diff * diff) as u64;
            }
        }
    }

    ssd
}

fn pixel_ssd_nv12_core(
    pixuv1: &[Pixel], stride1: usize, pixuv2: &[Pixel], stride2: usize, width: u32, height: u32,
) -> (u64, u64) {
    let mut ssd_u: u64 = 0;
    let mut ssd_v: u64 = 0;

    for y in 0..height {
        for x in 0..width {
            let index1 = (y as usize) * stride1 + (x as usize) * 2;
            let index2 = (y as usize) * stride2 + (x as usize) * 2;
            let du = pixuv1[index1] as i32 - pixuv2[index2] as i32;
            let dv = pixuv1[index1 + 1] as i32 - pixuv2[index2 + 1] as i32;
            ssd_u += (du * du) as u64;
            ssd_v += (dv * dv) as u64;
        }
    }

    (ssd_u, ssd_v)
}
