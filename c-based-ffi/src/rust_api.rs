use std::{ffi::CString, os::raw::c_char};

type Callback = unsafe extern "C" fn(n: u32);

type FnCommit = unsafe extern "C" fn(idx: u16);
type FnVoid = unsafe extern "C" fn();
type FnSetState = unsafe extern "C" fn(
    preedit: *const i8,
    candidates: *const *const i8,
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

        let cstrs: Vec<_> = candidates
            .iter()
            .map(|candidate| {
                CString::new(candidate.as_str()).expect("Failed to create C String from *const u8.")
            })
            .collect();

        let cstr_ptrs: Vec<_> = cstrs
            .iter() // do NOT into_iter()
            .map(|arg| arg.as_ptr())
            .collect();

        let cstr_ptrs_ptr = cstr_ptrs.as_ptr();

        let cnt = candidates.len();

        set_state(preedit_ptr, cstr_ptrs_ptr, lens_ptr, cnt);
        free_char_ptr(preedit_ptr);
    }
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
