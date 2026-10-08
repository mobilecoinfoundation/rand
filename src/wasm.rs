// Copyright (c) 2018-2026 The MobileCoin Foundation

use rand::rngs::SysRng;
use rand_core::UnwrapErr;

pub type McRng = UnwrapErr<SysRng>;
