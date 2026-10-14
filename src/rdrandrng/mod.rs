// Copyright (c) 2018-2026 The MobileCoin Foundation

// Note: This module is only expected to compile on x86 and x86_64

use core::convert::Infallible;
use rand_core::{utils, TryCryptoRng, TryRng};

mod retry;

// An implementation of TryRng which wraps calls to the RDRAND instruction
// Should work in enclave and out of enclave with no changes
#[derive(Default)]
pub struct McRng;

impl TryCryptoRng for McRng {}

impl TryRng for McRng {
    type Error = Infallible;

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(retry::next_rdrand_u32_or_panic())
    }

    // On x86_64 use the rdrand64_step instruction,
    // on x86 use `utils::next_u64_via_u32` which generically makes a u64 from
    // two u32s
    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        #[cfg(target_arch = "x86")]
        return utils::next_u64_via_u32(self);
        #[cfg(target_arch = "x86_64")]
        return Ok(retry::next_rdrand_u64_or_panic());
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        utils::fill_bytes_via_next_word(dest, || self.try_next_u64())
    }
}
