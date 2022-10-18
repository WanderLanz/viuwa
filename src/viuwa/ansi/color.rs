use std::fmt;

use image::{Luma, Pixel, Rgb};

use crate::viuwa::ColorAttributes;
macro_rules! set_fg {
        (RGB) => {
                csi!("38;2;{};{};{}m")
        };
        (256) => {
                csi!("38;5;{}m")
        };
}
macro_rules! set_bg {
        (RGB) => {
                csi!("48;2;{};{};{}m")
        };
        (256) => {
                csi!("48;5;{}m")
        };
}

/// Get the closest 8-bit color to the given 24-bit grayscale value.
#[inline]
pub fn gray_to_256(c: u8) -> u8 { GRAY_TO_256[c as usize] }

pub const MAP_0_100_DIST: f32 = 584970.0 / 100.0;
/// Get the closest 8-bit color to the given 24-bit color.
pub fn rgb_to_256(c: &[u8; 3], ca: &ColorAttributes) -> u8 {
        let xyz = rgb_xyz_256(c);
        let luma = gray_to_256(luma(c));
        if dist(c, &EIGHT_BIT_PALETTE[luma as usize]) + ca.luma_correct < dist(c, &EIGHT_BIT_PALETTE[xyz as usize]) {
                luma
        } else {
                xyz
        }
}

/// Get the luma of the given 24-bit color (sRGB -> Luma).
pub fn luma(c: &[u8; 3]) -> u8 { (((c[0] as u32 * 2126) + (c[1] as u32 * 7152) + (c[2] as u32 * 722)) / 10000) as u8 }

/// Get the distance between two 24-bit colors.
/// 0..=584970
pub const fn dist(a: &[u8; 3], b: &[u8; 3]) -> u32 {
        let r_ = (a[0] as u32 + b[0] as u32) / 2;
        let r = (a[0] as u32).abs_diff(b[0] as u32).pow(2);
        let g = (a[1] as u32).abs_diff(b[1] as u32).pow(2);
        let b = (a[2] as u32).abs_diff(b[2] as u32).pow(2);
        (((512 + r_) * r) >> 8) + (4 * g) + (((767 - r_) * b) >> 8)
}

const MAP_0_255_0_5: f32 = 5.0 / 255.0;
/// Get the closest 8-bit color in the 6x6x6 cube to the given 24-bit color.
pub fn rgb_xyz_256(c: &[u8; 3]) -> u8 {
        (((c[0] as f32 * MAP_0_255_0_5).round() as u8 * 36)
                + ((c[1] as f32 * MAP_0_255_0_5).round() as u8 * 6)
                + (c[2] as f32 * MAP_0_255_0_5).round() as u8)
                + 16
}

/// 256-color palette as 24-bit RGB values.
#[rustfmt::skip]
pub static EIGHT_BIT_PALETTE: [[u8;3]; 256] = [
        // unused, because they can be overriden
        [0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],[0x00, 0x00, 0x00],

        // 6×6×6 cube, RGB = XYZ
        [0x00,0x00,0x00], [0x00,0x00,0x5F], [0x00,0x00,0x87], [0x00,0x00,0xAF], [0x00,0x00,0xD7], [0x00,0x00,0xFF],
        [0x00,0x5F,0x00], [0x00,0x5F,0x5F], [0x00,0x5F,0x87], [0x00,0x5F,0xAF], [0x00,0x5F,0xD7], [0x00,0x5F,0xFF],
        [0x00,0x87,0x00], [0x00,0x87,0x5F], [0x00,0x87,0x87], [0x00,0x87,0xAF], [0x00,0x87,0xD7], [0x00,0x87,0xFF],
        [0x00,0xAF,0x00], [0x00,0xAF,0x5F], [0x00,0xAF,0x87], [0x00,0xAF,0xAF], [0x00,0xAF,0xD7], [0x00,0xAF,0xFF],
        [0x00,0xD7,0x00], [0x00,0xD7,0x5F], [0x00,0xD7,0x87], [0x00,0xD7,0xAF], [0x00,0xD7,0xD7], [0x00,0xD7,0xFF],
        [0x00,0xFF,0x00], [0x00,0xFF,0x5F], [0x00,0xFF,0x87], [0x00,0xFF,0xAF], [0x00,0xFF,0xD7], [0x00,0xFF,0xFF],

        [0x5F,0x00,0x00], [0x5F,0x00,0x5F], [0x5F,0x00,0x87], [0x5F,0x00,0xAF], [0x5F,0x00,0xD7], [0x5F,0x00,0xFF],
        [0x5F,0x5F,0x00], [0x5F,0x5F,0x5F], [0x5F,0x5F,0x87], [0x5F,0x5F,0xAF], [0x5F,0x5F,0xD7], [0x5F,0x5F,0xFF],
        [0x5F,0x87,0x00], [0x5F,0x87,0x5F], [0x5F,0x87,0x87], [0x5F,0x87,0xAF], [0x5F,0x87,0xD7], [0x5F,0x87,0xFF],
        [0x5F,0xAF,0x00], [0x5F,0xAF,0x5F], [0x5F,0xAF,0x87], [0x5F,0xAF,0xAF], [0x5F,0xAF,0xD7], [0x5F,0xAF,0xFF],
        [0x5F,0xD7,0x00], [0x5F,0xD7,0x5F], [0x5F,0xD7,0x87], [0x5F,0xD7,0xAF], [0x5F,0xD7,0xD7], [0x5F,0xD7,0xFF],
        [0x5F,0xFF,0x00], [0x5F,0xFF,0x5F], [0x5F,0xFF,0x87], [0x5F,0xFF,0xAF], [0x5F,0xFF,0xD7], [0x5F,0xFF,0xFF],

        [0x87,0x00,0x00], [0x87,0x00,0x5F], [0x87,0x00,0x87], [0x87,0x00,0xAF], [0x87,0x00,0xD7], [0x87,0x00,0xFF],
        [0x87,0x5F,0x00], [0x87,0x5F,0x5F], [0x87,0x5F,0x87], [0x87,0x5F,0xAF], [0x87,0x5F,0xD7], [0x87,0x5F,0xFF],
        [0x87,0x87,0x00], [0x87,0x87,0x5F], [0x87,0x87,0x87], [0x87,0x87,0xAF], [0x87,0x87,0xD7], [0x87,0x87,0xFF],
        [0x87,0xAF,0x00], [0x87,0xAF,0x5F], [0x87,0xAF,0x87], [0x87,0xAF,0xAF], [0x87,0xAF,0xD7], [0x87,0xAF,0xFF],
        [0x87,0xD7,0x00], [0x87,0xD7,0x5F], [0x87,0xD7,0x87], [0x87,0xD7,0xAF], [0x87,0xD7,0xD7], [0x87,0xD7,0xFF],
        [0x87,0xFF,0x00], [0x87,0xFF,0x5F], [0x87,0xFF,0x87], [0x87,0xFF,0xAF], [0x87,0xFF,0xD7], [0x87,0xFF,0xFF],

        [0xAF,0x00,0x00], [0xAF,0x00,0x5F], [0xAF,0x00,0x87], [0xAF,0x00,0xAF], [0xAF,0x00,0xD7], [0xAF,0x00,0xFF],
        [0xAF,0x5F,0x00], [0xAF,0x5F,0x5F], [0xAF,0x5F,0x87], [0xAF,0x5F,0xAF], [0xAF,0x5F,0xD7], [0xAF,0x5F,0xFF],
        [0xAF,0x87,0x00], [0xAF,0x87,0x5F], [0xAF,0x87,0x87], [0xAF,0x87,0xAF], [0xAF,0x87,0xD7], [0xAF,0x87,0xFF],
        [0xAF,0xAF,0x00], [0xAF,0xAF,0x5F], [0xAF,0xAF,0x87], [0xAF,0xAF,0xAF], [0xAF,0xAF,0xD7], [0xAF,0xAF,0xFF],
        [0xAF,0xD7,0x00], [0xAF,0xD7,0x5F], [0xAF,0xD7,0x87], [0xAF,0xD7,0xAF], [0xAF,0xD7,0xD7], [0xAF,0xD7,0xFF],
        [0xAF,0xFF,0x00], [0xAF,0xFF,0x5F], [0xAF,0xFF,0x87], [0xAF,0xFF,0xAF], [0xAF,0xFF,0xD7], [0xAF,0xFF,0xFF],

        [0xD7,0x00,0x00], [0xD7,0x00,0x5F], [0xD7,0x00,0x87], [0xD7,0x00,0xAF], [0xD7,0x00,0xD7], [0xD7,0x00,0xFF],
        [0xD7,0x5F,0x00], [0xD7,0x5F,0x5F], [0xD7,0x5F,0x87], [0xD7,0x5F,0xAF], [0xD7,0x5F,0xD7], [0xD7,0x5F,0xFF],
        [0xD7,0x87,0x00], [0xD7,0x87,0x5F], [0xD7,0x87,0x87], [0xD7,0x87,0xAF], [0xD7,0x87,0xD7], [0xD7,0x87,0xFF],
        [0xD7,0xAF,0x00], [0xD7,0xAF,0x5F], [0xD7,0xAF,0x87], [0xD7,0xAF,0xAF], [0xD7,0xAF,0xD7], [0xD7,0xAF,0xFF],
        [0xD7,0xD7,0x00], [0xD7,0xD7,0x5F], [0xD7,0xD7,0x87], [0xD7,0xD7,0xAF], [0xD7,0xD7,0xD7], [0xD7,0xD7,0xFF],
        [0xD7,0xFF,0x00], [0xD7,0xFF,0x5F], [0xD7,0xFF,0x87], [0xD7,0xFF,0xAF], [0xD7,0xFF,0xD7], [0xD7,0xFF,0xFF],

        [0xFF,0x00,0x00], [0xFF,0x00,0x5F], [0xFF,0x00,0x87], [0xFF,0x00,0xAF], [0xFF,0x00,0xD7], [0xFF,0x00,0xFF],
        [0xFF,0x5F,0x00], [0xFF,0x5F,0x5F], [0xFF,0x5F,0x87], [0xFF,0x5F,0xAF], [0xFF,0x5F,0xD7], [0xFF,0x5F,0xFF],
        [0xFF,0x87,0x00], [0xFF,0x87,0x5F], [0xFF,0x87,0x87], [0xFF,0x87,0xAF], [0xFF,0x87,0xD7], [0xFF,0x87,0xFF],
        [0xFF,0xAF,0x00], [0xFF,0xAF,0x5F], [0xFF,0xAF,0x87], [0xFF,0xAF,0xAF], [0xFF,0xAF,0xD7], [0xFF,0xAF,0xFF],
        [0xFF,0xD7,0x00], [0xFF,0xD7,0x5F], [0xFF,0xD7,0x87], [0xFF,0xD7,0xAF], [0xFF,0xD7,0xD7], [0xFF,0xD7,0xFF],
        [0xFF,0xFF,0x00], [0xFF,0xFF,0x5F], [0xFF,0xFF,0x87], [0xFF,0xFF,0xAF], [0xFF,0xFF,0xD7], [0xFF,0xFF,0xFF],

        // extra grayscale
        [0x08,0x08,0x08], [0x12,0x12,0x12], [0x1C,0x1C,0x1C], [0x26,0x26,0x26], [0x30,0x30,0x30], [0x3A,0x3A,0x3A],
        [0x44,0x44,0x44], [0x4E,0x4E,0x4E], [0x58,0x58,0x58], [0x62,0x62,0x62], [0x6C,0x6C,0x6C], [0x76,0x76,0x76],
        [0x80,0x80,0x80], [0x8A,0x8A,0x8A], [0x94,0x94,0x94], [0x9E,0x9E,0x9E], [0xA8,0xA8,0xA8], [0xB2,0xB2,0xB2],
        [0xBC,0xBC,0xBC], [0xC6,0xC6,0xC6], [0xD0,0xD0,0xD0], [0xDA,0xDA,0xDA], [0xE4,0xE4,0xE4], [0xEE,0xEE,0xEE],
];

/// Closest 256 color to a given grayscale value
// thanks to [ansi_colours](https://crates.io/crates/ansi_colours)
#[rustfmt::skip]
pub static GRAY_TO_256: [u8; 256] = [
        16,  16,  16,  16,  16, 232, 232, 232,
        232, 232, 232, 232, 232, 232, 233, 233,
        233, 233, 233, 233, 233, 233, 233, 233,
        234, 234, 234, 234, 234, 234, 234, 234,
        234, 234, 235, 235, 235, 235, 235, 235,
        235, 235, 235, 235, 236, 236, 236, 236,
        236, 236, 236, 236, 236, 236, 237, 237,
        237, 237, 237, 237, 237, 237, 237, 237,
        238, 238, 238, 238, 238, 238, 238, 238,
        238, 238, 239, 239, 239, 239, 239, 239,
        239, 239, 239, 239, 240, 240, 240, 240,
        240, 240, 240, 240,  59,  59,  59,  59,
        59,  241, 241, 241, 241, 241, 241, 241,
        242, 242, 242, 242, 242, 242, 242, 242,
        242, 242, 243, 243, 243, 243, 243, 243,
        243, 243, 243, 244, 244, 244, 244, 244,
        244, 244, 244, 244, 102, 102, 102, 102,
        102, 245, 245, 245, 245, 245, 245, 246,
        246, 246, 246, 246, 246, 246, 246, 246,
        246, 247, 247, 247, 247, 247, 247, 247,
        247, 247, 247, 248, 248, 248, 248, 248,
        248, 248, 248, 248, 145, 145, 145, 145,
        145, 249, 249, 249, 249, 249, 249, 250,
        250, 250, 250, 250, 250, 250, 250, 250,
        250, 251, 251, 251, 251, 251, 251, 251,
        251, 251, 251, 252, 252, 252, 252, 252,
        252, 252, 252, 252, 188, 188, 188, 188,
        188, 253, 253, 253, 253, 253, 253, 254,
        254, 254, 254, 254, 254, 254, 254, 254,
        254, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 231,
        231, 231, 231, 231, 231, 231, 231, 231,
];

pub trait AnsiPixel: Pixel<Subpixel = u8> {
        fn fg_24b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result;
        fn bg_24b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result;
        fn fg_8b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result;
        fn bg_8b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result;
}

impl AnsiPixel for Rgb<u8> {
        #[inline]
        fn fg_24b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                write!(writer, set_fg!(RGB), self.0[0], self.0[1], self.0[2])
        }
        #[inline]
        fn bg_24b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                write!(writer, set_bg!(RGB), self.0[0], self.0[1], self.0[2])
        }
        #[inline]
        fn fg_8b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result {
                write!(writer, set_fg!(256), rgb_to_256(&self.0, color_attrs))
        }
        #[inline]
        fn bg_8b<W: fmt::Write>(&self, writer: &mut W, color_attrs: &ColorAttributes) -> fmt::Result {
                write!(writer, set_bg!(256), rgb_to_256(&self.0, color_attrs))
        }
}

impl AnsiPixel for Luma<u8> {
        #[inline]
        fn fg_24b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                let c = self.0[0];
                write!(writer, set_fg!(RGB), c, c, c)
        }
        #[inline]
        fn bg_24b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                let c = self.0[0];
                write!(writer, set_bg!(RGB), c, c, c)
        }
        #[inline]
        fn fg_8b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                write!(writer, set_fg!(256), gray_to_256(self.0[0]))
        }
        #[inline]
        fn bg_8b<W: fmt::Write>(&self, writer: &mut W, _: &ColorAttributes) -> fmt::Result {
                write!(writer, set_bg!(256), gray_to_256(self.0[0]))
        }
}
