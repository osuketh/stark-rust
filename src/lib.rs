#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std::os::raw::c_uint;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn execute_fsrs(a: c_uint, b: c_uint, sec_prams: c_uint) {
    unsafe {
        root::execute(a, b, sec_prams);
    }
}
