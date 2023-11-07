//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022-2023 Evan Cox <evanacox00@gmail.com>. All rights reserved. //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

/// An implementation of the Xoshiro random number generator algorithm.
///
/// It has 256-bits of state, and produces 64-bit outputs.
pub struct Xoshiro256 {
    state: [u64; 4],
}

impl Xoshiro256 {
    /// Initializes the random-number generator with a given state.
    ///
    /// This seed should ideally put entropy in all 256 bits of state,
    /// or the output may not be particularly great.
    pub fn with_seed(seed: [u64; 4]) -> Self {
        Self { state: seed }
    }

    /// Takes the given seed and XORs it with a decent existing
    /// seed, effectively turns a terrible seed and turns it into
    /// a less terrible seed.
    pub fn with_seed_xor(seed: [u64; 4]) -> Self {
        let mut instance = Self::default();

        instance.state[0] ^= seed[0];
        instance.state[1] ^= seed[1];
        instance.state[2] ^= seed[2];
        instance.state[3] ^= seed[3];

        instance
    }

    fn rotate_left(x: u64, k: u64) -> u64 {
        (x << k) | (x >> (64 - k))
    }

    /// Produces the next 64-bit output from the hasher.
    ///
    /// This is relatively fast, and completely deterministic based
    /// on the seed and the previous number of calls to [`Self::next`].
    pub fn next(&mut self) -> u64 {
        let result = Self::rotate_left(self.state[1], 7).wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = Self::rotate_left(self.state[3], 45);

        result
    }
}

impl Default for Xoshiro256 {
    fn default() -> Self {
        Self {
            state: [
                // this is just hex for the first few digits of pi
                // in hex, nothing special going on
                0x243F_6A88_85A3_08D3,
                0x1319_8A2E_0370_7344,
                0xA409_3822_299F_31D0,
                0x082E_FA98_EC4E_6C89,
            ],
        }
    }
}
