use godot::prelude::*;

struct ELAU;

#[gdextension]
unsafe impl ExtensionLibrary for ELAU {}

use crate::naudr::curves_from_data;
pub mod naudr;