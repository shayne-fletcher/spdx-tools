#![allow(clippy::let_unit_value)]

use lrlex::lrlex_mod;

lrlex_mod!("spdx.l");

pub use spdx_l::*;
