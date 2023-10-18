//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

//! Kernel-level serial (UART) driver that integrates with `log`.
//!
//! This is intended to be linked directly into the kernel.

use crate::utility::{KSpinFairMutex, KSpinOnceCell};
use core::arch::asm;
use core::fmt::Write;
use core::hint;
use ksupport::sync::BasicMutex;

/// A UART serial backend that can be used for `log`.
///
/// This is the generic trait, the actual implementation is chosen
/// at build time based on the target architecture.
pub trait SerialBackend: Write {
    /// Initializes the serial port correctly
    fn init(&mut self);

    /// Writes a single byte to the serial backend.
    fn send(&mut self, byte: u8);

    /// Reads a single byte from the serial backend
    fn recv(&mut self) -> u8;
}

/// Wraps a standard x86-64 serial port (using `inb` and `outb`).
///
/// The port is expected to be compatible with
///
/// This is not able to be used in user-mode due to privileged instructions,
/// and must be kept thread and interrupt safe.
#[cfg(target_arch = "x86_64")]
pub struct SerialPortX86_64 {
    port: u16,
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;

    asm!("in al, dx", out("al") value, in("dx") port);

    value
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value);
}

#[cfg(target_arch = "x86_64")]
impl SerialPortX86_64 {
    /// Creates a serial port with a given port number.
    ///
    /// # Safety
    /// The given port must actually be a serial port for the current CPU.
    ///
    /// The port must also be initialized (via [`SerialBackend::init`] before
    /// any calls to [`SerialBackend::send`] or [`SerialBackend::recv`] are made,
    /// or they not work properly.
    #[inline(always)]
    pub const unsafe fn with_port(port: u16) -> Self {
        Self { port }
    }

    /// Creates a serial port with the default COM1 port (`0x3F8`).
    ///
    /// # Safety
    /// All the requirements lined out in [`Self::with_port`] apply.
    #[inline(always)]
    pub const unsafe fn default_com1() -> Self {
        Self::with_port(0x3F8)
    }

    // data port (read-write)
    #[inline(always)]
    const fn port_data(&self) -> u16 {
        self.port + 0
    }

    // interrupt enable port (write-only)
    #[inline(always)]
    const fn port_interrupt_enable(&self) -> u16 {
        self.port + 1
    }

    // fifo control port (write-only)
    #[inline(always)]
    const fn port_fifo_control(&self) -> u16 {
        self.port + 2
    }

    // line control port (write-only)
    #[inline(always)]
    const fn port_line_ctrl(&self) -> u16 {
        self.port + 3
    }

    // modem control port (write-only)
    #[inline(always)]
    const fn port_modem_ctrl(&self) -> u16 {
        self.port + 4
    }

    // line status port (read-only)
    #[inline(always)]
    const fn port_line_status(&self) -> u16 {
        self.port + 5
    }

    #[inline(always)]
    fn is_data_ready(&self) -> bool {
        unsafe {
            // lsb is 0 or 1 depending on if there's data to be read
            (inb(self.port_line_status()) & 1) != 0
        }
    }

    #[inline(always)]
    fn is_transmission_buffer_empty(&self) -> bool {
        // bit 5 is 0 or 1 depending on if data can be transmitted
        unsafe { (inb(self.port_line_status()) & 0b100000) != 0 }
    }
}

#[cfg(target_arch = "x86_64")]
impl SerialBackend for SerialPortX86_64 {
    fn init(&mut self) {
        unsafe {
            outb(self.port_interrupt_enable(), 0x00); // Disable all interrupts
            outb(self.port_line_ctrl(), 0x80); // Enable DLAB (set baud rate divisor)
            outb(self.port_data(), 0x03); // Set divisor to 3 (lo byte) 38400 baud
            outb(self.port_interrupt_enable(), 0x00); //                  (hi byte)
            outb(self.port_line_ctrl(), 0x03); // 8 bits, no parity, one stop bit
            outb(self.port_fifo_control(), 0xC7); // Enable FIFO, clear them, with 14-byte threshold
            outb(self.port_modem_ctrl(), 0x0F); // set it in normal operation mode
            outb(self.port_interrupt_enable(), 0x01); // enable interrupts
        }
    }

    fn send(&mut self, byte: u8) {
        while !self.is_transmission_buffer_empty() {
            hint::spin_loop();
        }

        unsafe { outb(self.port_data(), byte) }
    }

    fn recv(&mut self) -> u8 {
        while !self.is_data_ready() {
            hint::spin_loop();
        }

        unsafe { inb(self.port_data()) }
    }
}

#[cfg(target_arch = "x86_64")]
impl Write for SerialPortX86_64 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }

        Ok(())
    }
}

#[cfg(target_arch = "x86_64")]
type SerialImpl = SerialPortX86_64;

#[cfg(not(target_arch = "x86_64"))]
type SerialImpl = !;

/// The underlying serial port implementation for the specific
/// CPU being targeted when the OS is built.
pub type SerialPort = SerialImpl;

static SERIAL_PORT: KSpinOnceCell<KSpinFairMutex<SerialPort>> = KSpinOnceCell::uninit();

/// Initializes the framebuffer with a given function.
///
/// This is meant to be called from the boot code for a given architecture,
/// where the framebuffer is initialized and then is ready to use from then on.
#[inline]
pub fn serial_init(f: impl FnOnce() -> SerialPort) {
    let _ = SERIAL_PORT.set(KSpinFairMutex::new(f()));

    // initialize the port
    SERIAL_PORT.get().lock().init();
}

/// Returns a reference to the lock that guards the serial port.
#[inline]
pub fn serial() -> &'static KSpinFairMutex<SerialPort> {
    SERIAL_PORT.get()
}
