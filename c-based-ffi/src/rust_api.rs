#![feature(vec_into_raw_parts)]

use std::{ffi::CString, os::raw::c_char};

type Callback = unsafe extern "C" fn(n: u32);

type FnCommit = unsafe extern "C" fn(idx: u16);
type FnVoid = unsafe extern "C" fn();
type FnSetState = unsafe extern "C" fn(
    preedit: *const i8,
    candidates: *mut *mut i8,
    lens: *const u16,
    cnt: usize,
);

#[no_mangle]
pub extern "C" fn r_run_callbacks(
    commit: FnCommit,
    page_up: FnVoid,
    page_down: FnVoid,
    set_state: FnSetState,
) {
    unsafe {
        commit(5);

        page_up();

        page_down();

        let preedit_ptr = str_to_char_ptr(&"abc");

        let lens = vec![1, 2, 3, 4];
        let lens_ptr = lens.as_ptr();

        let candidates = vec![
            "一".to_owned(),
            "键".to_owned(),
            "三".to_owned(),
            "连".to_owned(),
        ];

        let (cstring_array, cstring_len, cstring_cap) = string_vec_to_cstring_array(&candidates);

        let cnt = candidates.len();

        set_state(preedit_ptr, cstring_array, lens_ptr, cnt);
        free_char_ptr(preedit_ptr);
        free_cstring_array(cstring_array, cstring_len, cstring_cap)
    }
}

unsafe fn string_vec_to_cstring_array(input: &Vec<String>) -> (*mut *mut c_char, usize, usize) {
    let ptrs: Vec<*mut c_char> = input
        .iter()
        .map(|string| str_to_char_ptr(string.as_str()))
        .collect();
    ptrs.into_raw_parts()
}

unsafe fn free_cstring_array(ptr: *mut *mut c_char, len: usize, cap: usize) {
    let _ = Vec::from_raw_parts(ptr, len, cap);
}

unsafe fn str_to_char_ptr(input: &str) -> *mut c_char {
    let char_ptr = CString::new(input)
        .expect("Failed to create C string from &str.")
        .into_raw();
    return char_ptr;
}

unsafe fn free_char_ptr(ptr: *mut c_char) {
    let _ = CString::from_raw(ptr);
}

#[no_mangle]
pub extern "C" fn r_add_cb(a: u32, b: u32, cb: Callback) -> u32 {
    unsafe {
        cb(a + b);
    }
    a + b
}
