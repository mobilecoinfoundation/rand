// Copyright (c) 2018-2026 The MobileCoin Foundation

use core::convert::Infallible;
use rand::{rng, TryCryptoRng, TryRng};

#[derive(Clone, Debug, Default)]
pub struct McRng;

impl TryRng for McRng {
    type Error = Infallible;

    #[inline(always)]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        rng().try_next_u32()
    }

    #[inline(always)]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        rng().try_next_u64()
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        rng().try_fill_bytes(dest)
    }
}

impl TryCryptoRng for McRng {}
