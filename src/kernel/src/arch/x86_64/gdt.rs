//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

/// The CPU privilege level that the selector encodes.
///
/// Only ring0 and ring3 are supported, the other 2 are not used
/// by the operating system.
#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum Privilege {
    Ring0 = 0b00,
    Ring3 = 0b11,
}

/// Models a segment selector, as defined in the Intel manuals
/// and used by the IDT.
#[repr(transparent)]
pub struct SegmentSelector {
    raw: u16,
}

impl SegmentSelector {
    /// Creates a segment selector that uses the GDT, and
    /// uses the entry at a given index and privilege level.
    pub const fn for_gdt(index: u8, privilege: Privilege) -> Self {
        Self {
            // see section 3.4.2 in the Intel Manual.
            // the GDT index is bits 3..15,
            // GDT or LDT flag is bit 2 (we chose GDT)
            // ring level is bit 0..1
            raw: ((index as u16) << 3) | (privilege as u16),
        }
    }
}

/// Returns a segment selector that maps to the kernel's code segment.
///
/// This is highly specific to Limine.
pub const fn cs() -> SegmentSelector {
    SegmentSelector::for_gdt(5, Privilege::Ring0)
}

/// An invalid selector referring to the null segment.
pub const fn null() -> SegmentSelector {
    SegmentSelector::for_gdt(0, Privilege::Ring0)
}
