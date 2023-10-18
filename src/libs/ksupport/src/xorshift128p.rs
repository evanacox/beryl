//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

/// An implementation of the Xorshift random number generator algorithm.
///
/// It has 128-bits of state, and produces 64-bit outputs.
pub struct Xorshift128Plus {
    state: [u64; 2],
}

impl Xorshift128Plus {
    /// Initializes the random-number generator with a given state.
    ///
    /// This seed should ideally put entropy in all 128 bits of state,
    /// or the output may not be particularly great.
    pub fn with_seed(seed: [u64; 2]) -> Self {
        Self { state: seed }
    }

    /// Takes the given seed and XORs it with a decent existing
    /// seed, effectively turns a terrible seed and turns it into
    /// a less terrible seed.
    pub fn with_seed_xor(seed: [u64; 2]) -> Self {
        let mut instance = Self::default();

        instance.state[0] ^= seed[0];
        instance.state[1] ^= seed[1];

        instance
    }

    /// Produces the next 64-bit output from the hasher.
    ///
    /// This is relatively fast, and completely deterministic based
    /// on the seed and the previous number of calls to [`Self::next`].
    pub fn next(&mut self) -> u64 {
        let mut t = self.state[0];
        let s = self.state[1];
        self.state[0] = s;

        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);

        self.state[1] = t;

        t + s
    }
}

impl Default for Xorshift128Plus {
    fn default() -> Self {
        Self {
            state: [
                // this is just hex for some digits of pi in hex, nothing special going on
                0x243F_6A88_85A3_08D3,
                0x082E_FA98_EC4E_6C89,
            ],
        }
    }
}
