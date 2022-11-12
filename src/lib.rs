#![feature(fn_traits, unboxed_closures)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate bit_set;
extern crate bitintr;
extern crate speedy;

#[macro_use]
mod rtps;
mod base;
mod dds;

#[macro_use]
extern crate lazy_static;

/// Set locator IP address to 0
#[macro_export]
macro_rules! LOCATOR_ADDRESS_INVALID {
    ($a:expr) => {
        $a.iter_mut().for_each(|m| *m = 0)
    };
}
