//use std::ffi::{CStr, };
//use std::os::raw::c_char;
//use std::sync::Mutex;

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn start_game() -> i32;
}

#[cfg(target_os = "emscripten")]
extern "C" {
    pub fn end_game() -> i32;
}
