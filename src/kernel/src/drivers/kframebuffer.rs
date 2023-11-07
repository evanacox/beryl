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

#[cfg(target_arch = "x86_64")]
use bootloader_api::info::{FrameBuffer, PixelFormat};

/// Models a 4-byte color with red, green, blue, and an optional alpha channel.
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
}

/// Represents possible framebuffer color formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorFormat {
    /// One byte red, one byte blue, one byte green
    Rgb,
    /// One byte blue, one byte green, one byte red
    Bgr,
    /// One grayscale byte
    Grayscale,
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
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        let at = x * self.bytes_per_pixel + y * self.pitch;

        match self.format {
            ColorFormat::Rgb => {
                self.raw[at] = color.r;
                self.raw[at + 1] = color.g;
                self.raw[at + 2] = color.b;
            }
            ColorFormat::Bgr => {
                self.raw[at] = color.b;
                self.raw[at + 1] = color.g;
                self.raw[at + 2] = color.r;
            }
            ColorFormat::Grayscale => {
                self.raw[at] = color.grayscale();
            }
        }
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
}

#[cfg(target_arch = "x86_64")]
impl From<&'static mut FrameBuffer> for LinearFramebuffer {
    fn from(buf: &'static mut FrameBuffer) -> Self {
        let info = buf.info();
        let format = match info.pixel_format {
            PixelFormat::Rgb => ColorFormat::Rgb,
            PixelFormat::Bgr => ColorFormat::Bgr,
            PixelFormat::U8 => ColorFormat::Grayscale,
            PixelFormat::Unknown { .. } => ColorFormat::Grayscale,
            _ => ColorFormat::Grayscale,
        };

        Self {
            raw: buf.buffer_mut(),
            width: info.width,
            height: info.height,
            pitch: info.stride,
            bytes_per_pixel: info.bytes_per_pixel,
            format,
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
