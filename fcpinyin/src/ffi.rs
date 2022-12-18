use std::{ffi::CString, os::raw::c_char, sync::Arc};

use crate::{
    fcitx5::{Engine, Fcitx5, FcitxKey, Table, UI},
    fcp::Fcp,
};

pub type FnCommit = unsafe extern "C" fn(idx: u16);
pub type FnVoid = unsafe extern "C" fn();
pub type FnCanPageUp = unsafe extern "C" fn() -> bool;
pub type FnSetCandidates = unsafe extern "C" fn(candidates: *mut *mut i8, cnt: usize);
pub type FnSetPreedit = unsafe extern "C" fn(preedit: *const i8);
pub type FnSetPage = unsafe extern "C" fn(idx: i32);

#[repr(C)]
pub struct FcpOpaque {
    fcp: Option<Arc<Fcp>>,
}

#[no_mangle]
pub extern "C" fn new_fcp() -> *mut FcpOpaque {
    Box::into_raw(Box::new(FcpOpaque {
        fcp: Some(Arc::new(Fcp::new())),
    }))
}

#[no_mangle]
pub extern "C" fn register_callbacks(
    opaque: *mut FcpOpaque,
    set_loading: FnVoid,
    set_candidates: FnSetCandidates,
    clear_candidates: FnVoid,
    set_preedit: FnSetPreedit,
    can_page_up: FnCanPageUp,
    page_up: FnVoid,
    page_down: FnVoid,
    prev: FnVoid,
    next: FnVoid,
    set_page: FnSetPage,
    commit: FnCommit,
    commit_preedit: FnSetPreedit,
    commit_candidate_by_fixed_key: FnVoid,
) {
    let ui = UI {
        set_loading,
        set_candidates,
        clear_candidates,
        set_preedit,
    };

    let table = Table {
        can_page_up,
        page_up,
        page_down,
        prev,
        next,
        set_page,
    };

    let engine = Engine {
        commit,
        commit_preedit,
        commit_candidate_by_fixed_key,
    };

    let fcitx5 = Fcitx5 { ui, table, engine };
    unsafe {
        (*opaque)
            .fcp
            .as_ref()
            .expect("fcp is no longer valid.")
            .set_fcitx5(fcitx5);
    }
}

#[no_mangle]
pub extern "C" fn on_key_press(opaque: *mut FcpOpaque, key: FcitxKey) -> bool {
    unsafe {
        (*opaque)
            .fcp
            .clone()
            .expect("fcp isn't valid.")
            .on_key_press(key)
    }
}

pub unsafe fn str_vec_to_cstring_array(input: Vec<&String>) -> (*mut *mut c_char, usize, usize) {
    let ptrs: Vec<*mut c_char> = input
        .into_iter()
        .map(|string| str_to_char_ptr(string.as_str()))
        .collect();
    ptrs.into_raw_parts()
}

pub unsafe fn free_cstring_array(ptr: *mut *mut c_char, len: usize, cap: usize) {
    let _ = Vec::from_raw_parts(ptr, len, cap);
}

pub unsafe fn str_to_char_ptr(input: &str) -> *mut c_char {
    let char_ptr = CString::new(input)
        .expect("Failed to create C string from &str.")
        .into_raw();
    return char_ptr;
}

pub unsafe fn free_char_ptr(ptr: *mut c_char) {
    let _ = CString::from_raw(ptr);
}
