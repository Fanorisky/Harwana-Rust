#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_transmute_annotations)]

use omp_gdk::Loader;
use inventory;

mod helper;
pub mod scripting;

pub mod core {
    pub use crate::scripting::functions::{
        Test_AddNumbers as AddNumbers,
    };
}

fn load_streamer_function() {
    scripting::load_functions();
}

inventory::submit! {
    Loader(load_streamer_function)
}
