// -*- mode: rust; coding: utf-8; -*-
//
// This file is part of curve25519-dalek.
// Copyright (c) 2016-2018 Isis Lovecruft, Henry de Valence
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

use core::ops::{Mul};
use packed_simd::{u32x8, IntoBits};

fn unpack_pair(_src: u32x8) -> (u32x8, u32x8) {
    unimplemented!()
}

fn repack_pair(x: u32x8, y: u32x8) -> u32x8 {
    unsafe {
        use core::arch::x86_64::_mm256_blend_epi32;
        use core::arch::x86_64::_mm256_shuffle_epi32;

        let x_shuffled = _mm256_shuffle_epi32(x.into_bits(), 0b11_01_10_00);
        let y_shuffled = _mm256_shuffle_epi32(y.into_bits(), 0b10_00_11_01);

        return _mm256_blend_epi32(x_shuffled, y_shuffled, 0b11001100).into_bits();
    }
}

pub struct FieldElement2625x4(pub(crate) [u32x8; 1]);

impl<'a, 'b> Mul<&'b FieldElement2625x4> for &'a FieldElement2625x4 {
    type Output = FieldElement2625x4;
    fn mul(self, rhs: &'b FieldElement2625x4) -> FieldElement2625x4 {
        let (y0, y1) = unpack_pair(rhs.0[0]);

        FieldElement2625x4([
            repack_pair(y0, y1),
        ])
    }
}