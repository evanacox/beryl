//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Kernel-level framebuffer driver.
//!
//! This driver is purely for debug purposes, and is meant to be replaced
//! by a user-mode graphics driver. For now, this is for CPU-level interaction
//! with the framebuffer as exposed by the bootloader.
//!
//! This is intended to be linked directly into the kernel.

use crate::utility::{KSpinFairMutex, KSpinOnceCell};
use core::mem;
use core::ptr;
use core::slice;

#[cfg(target_arch = "x86_64")]
use limine::Framebuffer;

/// Models a 4-byte color with red, green, blue, and an optional alpha channel.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    /// Red channel
    pub r: u8,
    /// Green channel
    pub g: u8,
    /// Blue channel
    pub b: u8,
    /// (unused for now) alpha channel
    pub a: u8,
}

impl Color {
    /// Creates an RGB color (with alpha = 255).
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }

    /// Approximates grayscale based on the (r, g, b) triple
    /// represented by the color.
    ///
    /// Only uses integer math, so it's only an approximation.
    #[inline]
    pub fn grayscale(self) -> u8 {
        // 0.30*R + 0.59*G + 0.11*B approximates grayscale conversion
        let red = ((self.r as u16 * 30) / 100) as u8;
        let green = ((self.g as u16 * 59) / 100) as u8;
        let blue = ((self.b as u16 * 11) / 100) as u8;

        red + green + blue
    }

    /// Returns a `u32` in RGBA format.
    #[inline]
    pub fn rgba(self) -> u32 {
        unsafe { mem::transmute(self) }
    }
}

/// Represents possible framebuffer color formats.
///
/// It compactly stores a set of shifts, and provides a trivial way
/// to "rearrange" RGB color `u32`s into valid pixels.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ColorFormat {
    rearranger: fn(Self, u32) -> u32,
    shifts: u32,
    masks: u32,
}

impl ColorFormat {
    /// Creates a new [`ColorFormat`].
    ///
    /// The `red`/`green`/`blue` are pairs of the form `(shift, mask)`, where the shift
    /// is the amount to shift the red value by, and the mask is the bitmask to mask a red
    /// value by before shifting it into the final value.
    ///
    /// Roughly, the final value is computed like this:
    /// ```
    /// let rgb: u32 = 0; // some RGB value
    /// let rearranged = (((rgb & 0xFF) & red_mask) << red_shift)
    ///                | ((((rgb >> 8) & 0xFF) & green_mask) << green_shift)
    ///                | ((((rgb >> 16) & 0xFF) & blue_mask) << blue_shift);
    /// ```
    pub const fn new(red: (u8, u8), green: (u8, u8), blue: (u8, u8)) -> Self {
        let (red_shift, red_mask) = red;
        let (green_shift, green_mask) = green;
        let (blue_shift, blue_mask) = blue;

        Self {
            rearranger: match (red, green, blue) {
                ((0, 8), (8, 8), (16, 8)) => Self::rearrange_rgb_into_rgb,
                ((8, 8), (16, 8), (0, 8)) => Self::rearrange_rgb_into_brg,
                _ => Self::rearrange_always_correct,
            },
            shifts: (red_shift as u32) | ((green_shift as u32) << 8) | ((blue_shift as u32) << 16),
            masks: (red_mask as u32) | ((green_mask as u32) << 8) | ((blue_mask as u32) << 16),
        }
    }

    /// Re-arranges the color `rgb` into a `u32` color.
    #[inline]
    pub fn rearrange(self, rgb: Color) -> u32 {
        (self.rearranger)(self, rgb.rgba())
    }

    // specialized method for rgb -> rgb, which is very common
    // the ideal is that the branch predictor can figure out where this
    // is jumping to and effectively remove the cost
    const fn rearrange_rgb_into_rgb(self, rgb: u32) -> u32 {
        rgb
    }

    // specialized method for rgb -> brg color, which is also very common
    //
    // constants allow compiler to optimize this a lot, i'm not bothered
    const fn rearrange_rgb_into_brg(self, rgb: u32) -> u32 {
        let red = rgb & 0xFF;
        let green = (rgb >> 8) & 0xFF;
        let blue = (rgb >> 16) & 0xFF;

        blue | (red << 8) | (green << 16)
    }

    // the fallback method that works with all values, not specialized
    const fn rearrange_always_correct(self, rgb: u32) -> u32 {
        let red = ((rgb & 0xFF) & self.red_mask()) << self.red_shift();
        let green = (((rgb >> 8) & 0xFF) & self.green_mask()) << self.green_shift();
        let blue = (((rgb >> 16) & 0xFF) & self.blue_mask()) << self.blue_shift();

        red | green | blue
    }

    #[inline]
    const fn red_mask(&self) -> u32 {
        self.masks & 0xFF
    }

    #[inline]
    const fn green_mask(&self) -> u32 {
        (self.masks >> 8) & 0xFF
    }

    #[inline]
    const fn blue_mask(&self) -> u32 {
        (self.masks >> 16) & 0xFF
    }

    #[inline]
    const fn red_shift(&self) -> u32 {
        self.shifts & 0xFF
    }

    #[inline]
    const fn green_shift(&self) -> u32 {
        (self.shifts >> 8) & 0xFF
    }

    #[inline]
    const fn blue_shift(&self) -> u32 {
        (self.shifts >> 16) & 0xFF
    }
}

/// Represents a hardware framebuffer provided by the hardware.
///
/// These are in true-color mode, not VGA mode.
pub struct LinearFramebuffer {
    raw: &'static mut [u8],
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
    format: ColorFormat,
}

impl LinearFramebuffer {
    /// Sets the pixel at (`x`, `y`) to `color`
    ///
    /// Exactly how this is done depends on the format being used
    /// by the framebuffer.
    #[inline]
    pub fn buffered_set(&mut self, buf: &mut [u8], x: usize, y: usize, color: Color) {
        let at = x * self.bytes_per_pixel + y * self.pitch;
        let raw = self.format.rearrange(color);
        let bytes = raw.to_le_bytes();

        for i in 0..4 {
            buf[at + i] = bytes[i];
        }
    }

    /// Copies the given buffer into the framebuffer, effectively
    /// writing to the screen.
    #[inline]
    pub fn buffered_write(&mut self, buf: &[u8]) {
        self.raw.copy_from_slice(buf);
    }

    /// Sets the framebuffer at exactly `offset` to `value`.
    ///
    /// This is directly accessing the underlying buffer, with no
    /// adjustments or anything done.
    #[inline]
    pub fn raw_set(&mut self, at: usize, value: u8) {
        self.raw[at] = value;
    }

    /// The number of horizontal pixels in a row, e.g. the 1920 in 1920x1080p
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// The number of vertical pixels in a row, e.g. the 1080 in 1920x1080p
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// The number of bytes between the start of one row to the start of another row
    #[inline]
    pub fn pitch(&self) -> usize {
        self.pitch
    }

    /// The number of bytes between the start of two pixels
    #[inline]
    pub fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_pixel
    }

    /// Returns a pointer to the raw framebuffer
    #[inline]
    pub fn raw_buffer(&mut self) -> *mut u8 {
        self.raw.as_mut_ptr()
    }

    /// Returns a slice that contains the entire framebuffer
    #[inline]
    pub fn full_raw_buffer(&mut self) -> &mut [u8] {
        self.raw
    }

    /// Returns the color format of the framebuffer
    #[inline]
    pub fn format(&self) -> ColorFormat {
        self.format
    }
}

#[inline]
const fn dynamic_bitmask_for_n_bits(n: u8) -> u8 {
    !((n != 0) as u8) & (!1 >> 8 - n)
}

#[cfg(target_arch = "x86_64")]
impl From<&'static mut Framebuffer> for LinearFramebuffer {
    fn from(buf: &'static mut Framebuffer) -> Self {
        let red_mask = dynamic_bitmask_for_n_bits(buf.red_mask_size);
        let green_mask = dynamic_bitmask_for_n_bits(buf.green_mask_size);
        let blue_mask = dynamic_bitmask_for_n_bits(buf.blue_mask_size);

        let length_bytes = buf.width * buf.height * (buf.bpp as u64 / 4);

        Self {
            raw: unsafe {
                slice::from_raw_parts_mut(buf.address.as_ptr().unwrap(), length_bytes as usize)
            },
            width: buf.width as usize,
            height: buf.height as usize,
            pitch: buf.pitch as usize,
            bytes_per_pixel: (buf.bpp as usize) / 4,
            format: ColorFormat::new(
                (buf.red_mask_shift, red_mask),
                (buf.green_mask_shift, green_mask),
                (buf.blue_mask_shift, blue_mask),
            ),
        }
    }
}

static FRAMEBUFFER: KSpinOnceCell<KSpinFairMutex<LinearFramebuffer>> = KSpinOnceCell::uninit();

/// Initializes the framebuffer with a given function.
///
/// This is meant to be called from the boot code for a given architecture,
/// where the framebuffer is initialized and then is ready to use from then on.
#[inline]
pub fn framebuffer_init(f: impl FnOnce() -> LinearFramebuffer) {
    let _ = FRAMEBUFFER.set(KSpinFairMutex::new(f()));
}

/// Returns a reference to the framebuffer.
#[inline]
pub fn framebuffer() -> &'static KSpinFairMutex<LinearFramebuffer> {
    FRAMEBUFFER.get()
}
