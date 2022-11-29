use std::{os::raw::c_char, ffi::CString};

use crate::fcitx5::{FcitxKey, Table, UI, Engine, Fcitx5};

pub type FnCommit = unsafe extern "C" fn(idx: u16);
pub type FnVoid = unsafe extern "C" fn();
pub type FnSetCandidates = unsafe extern "C" fn(candidates: *mut *mut i8, cnt: usize);
pub type FnSetPreedit = unsafe extern "C" fn(preedit: *const i8);

#[no_mangle]
pub extern "C" fn on_key_press(key: FcitxKey) {
    println!("Rust: {:#?}", key);
}

#[no_mangle]
pub extern "C" fn register_fn_commit(callback: FnCommit) {
    unsafe {
        callback(55);
    }
}

#[no_mangle]
pub extern "C" fn register_fn_void(callback: FnVoid) {
    unsafe {
        callback();
    }
}

#[no_mangle]
pub extern "C" fn register_fn_set_candidates(callback: FnSetCandidates) {
    let candidates = vec!["今天".to_owned(), "感觉".to_owned(), "怎么样".to_owned()];
    unsafe {
        let (ptr, len, cap) = string_vec_to_cstring_array(&candidates);
        let cnt = 3;
        callback(ptr, cnt);
        free_cstring_array(ptr, len, cap);
    }
}

#[no_mangle]
pub extern "C" fn register_fn_set_preedit(callback: FnSetPreedit) {
    let preedit = "abc".to_owned();
    unsafe {
        let ptr = str_to_char_ptr(&preedit);
        callback(ptr);
        free_char_ptr(ptr);
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
pub extern "C" fn register_callbacks(
    set_loading: FnVoid,
    set_candidates: FnSetCandidates,
    append_candidates: FnSetCandidates,
    set_preedit: FnSetPreedit,
    page_up: FnVoid,
    page_down: FnVoid,
    prev: FnVoid,
    next: FnVoid,
    commit: FnCommit,
    commit_candidate_by_fixed_key: FnVoid,
) {
    let ui = UI {
        set_loading,
        set_candidates,
        append_candidates,
        set_preedit,
    };

    let table = Table {
        page_up,
        page_down,
        prev,
        next,
    };

    let engine = Engine {
        commit,
        commit_candidate_by_fixed_key,
    };

    let fcitx5 = Fcitx5 { ui, table, engine };
}