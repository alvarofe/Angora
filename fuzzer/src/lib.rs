#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(duration_float)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_more;

mod branches;
mod cond_stmt;
mod depot;
pub mod executor;
mod mut_input;
mod search;
mod stats;
pub mod track;

mod fuzz_loop;
mod fuzz_main;
mod fuzz_type;

mod bind_cpu;
mod check_dep;
mod command;
mod tmpfs;

pub use crate::fuzz_main::fuzz_main;
