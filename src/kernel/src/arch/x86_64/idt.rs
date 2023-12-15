//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::arch::x86_64::gdt;
use crate::arch::x86_64::gdt::{Privilege, SegmentSelector};

/// The `options` field in an IDT entry.
///
/// This is all the misc. options for the interrupt subsystem,
/// and must be given for every interrupt.
#[derive(Copy, Clone, Debug)]
pub struct EntryOptions {
    raw: u16,
}

impl EntryOptions {
    const STACK_INDEX: u16 = 0b000;

    const RESERVED_BITS: u16 = 0b00000 << 3;

    const INTERRUPTS_DISABLED: u16 = 0b0 << 8;

    const MUST_BE_ONE: u16 = 0b111 << 9;

    const MUST_BE_ZERO: u16 = 0b0 << 12;

    const MINIMUM_PRIVILEGE_ZERO: u16 = (Privilege::Ring0 as u16) << 13;

    const NOT_PRESENT: u16 = 0b0 << 15;

    /// A "disabled" interrupts option value that is mostly zeroed out,
    /// except for the few bits that have to be one.
    pub const fn disabled_ring0() -> Self {
        Self {
            raw: Self::STACK_INDEX
                | Self::RESERVED_BITS
                | Self::INTERRUPTS_DISABLED
                | Self::MUST_BE_ONE
                | Self::MUST_BE_ZERO
                | Self::MINIMUM_PRIVILEGE_ZERO
                | Self::NOT_PRESENT,
        }
    }

    /// Returns a copy of [`Self`] with the present bit set to `present`.
    pub const fn with_present(self, present: bool) -> Self {
        const EVERYTHING_ELSE_MASK: u16 = 0b0111_1111_1111_1111;

        Self {
            raw: (self.raw & EVERYTHING_ELSE_MASK) | (present as u16) << 15,
        }
    }

    /// Returns a copy of [`Self`] with the privilege bits set to `privilege`.
    pub const fn with_privilege(self, privilege: Privilege) -> Self {
        const EVERYTHING_ELSE_MASK: u16 = 0b1001_1111_1111_1111;

        Self {
            raw: (self.raw & EVERYTHING_ELSE_MASK) | (privilege as u16) << 13,
        }
    }

    /// Returns a copy of [`Self`] with the interrupt bit set to `enabled`
    pub const fn with_interrupts_enabled(self, enabled: bool) -> Self {
        const EVERYTHING_ELSE_MASK: u16 = 0b1111_1110_1111_1111;

        Self {
            raw: (self.raw & EVERYTHING_ELSE_MASK) | (enabled as u16) << 8,
        }
    }
}

/// A single interrupt handler entry inside the IDT.
///
/// This can be directly read by the CPU as part of the IDT, no conversion
/// is necessary. An array of this type is valid as an IDT.
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct IDTEntry {
    isr_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    isr_mid: u16,
    isr_high: u32,
    __reserved: u32,
}

impl IDTEntry {
    /// Creates an empty IDT entry, used for interrupts that don't have handlers.
    ///
    /// These should be replaced with some sort of catch-all in the actual IDT,
    /// but this is valid for static initialization.
    pub const fn missing() -> Self {
        Self {
            isr_low: 0,
            gdt_selector: gdt::null(),
            options: EntryOptions::disabled_ring0(),
            isr_mid: 0,
            isr_high: 0,
            __reserved: 0,
        }
    }

    /// Creates an IDT entry that has a given handler function as the callback.
    ///
    /// This IDT entry is active.
    pub const fn with_handler(handler: extern "C" fn() -> !) -> Self {
        let address = handler as u64;
        let lo = (address & u16::MAX) as u16;
        let mi = ((address >> 16) & u16::MAX) as u16;
        let hi = ((address >> 32) & u32::MAX) as u32;

        Self {
            isr_low: lo,
            gdt_selector: gdt::cs(),
            options: EntryOptions::disabled_ring0().with_present(true),
            isr_mid: mi,
            isr_high: hi,
            __reserved: 0,
        }
    }
}
