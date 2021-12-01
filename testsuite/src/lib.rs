#![no_std]
#![cfg_attr(test, no_main)]

use ao_c2021 as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
