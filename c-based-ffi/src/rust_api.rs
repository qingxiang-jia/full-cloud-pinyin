use std::ffi::CString;

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

        let preedit = CString::new("abc").expect("CString::new failed");
        let preedit_ptr = preedit.as_ptr();

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
    }
}

#[no_mangle]
pub extern "C" fn r_add_cb(a: u32, b: u32, cb: Callback) -> u32 {
    unsafe {
        cb(a + b);
    }
    a + b
}
