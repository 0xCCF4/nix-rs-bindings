use nix_exprc_sys::{
    nix_alloc_value, nix_expr_eval_from_string, nix_gc_decref, nix_get_string, nix_libexpr_init,
    nix_state_create, nix_state_free, nix_store_free, nix_store_open, nix_value_force,
};
use std::ffi::{CStr, c_char, c_uint, c_void};
use std::ptr;

extern "C" fn my_get_string_cb(start: *const c_char, _n: c_uint, user_data: *mut c_void) {
    unsafe {
        *(user_data as *mut *mut c_char) = libc::strdup(start);
    }
}

// https://github.com/NixOS/nix/tree/master/src/external-api-docs#embedding-the-nix-evaluatornix_evaluator_example
fn main() {
    unsafe {
        nix_libexpr_init(ptr::null_mut());

        let store = nix_store_open(
            ptr::null_mut(),
            "dummy://\0" as *const str as *const c_char,
            ptr::null_mut(),
        );
        let state = nix_state_create(ptr::null_mut(), ptr::null_mut(), store);
        let value = nix_alloc_value(ptr::null_mut(), state);

        nix_expr_eval_from_string(
            ptr::null_mut(),
            state,
            "builtins.nixVersion\0".as_ptr() as *const c_char,
            ".\0".as_ptr() as *const c_char,
            value,
        );
        nix_value_force(ptr::null_mut(), state, value);

        let mut version: *mut c_char = ptr::null_mut();
        nix_get_string(
            ptr::null_mut(),
            value,
            Some(my_get_string_cb),
            &mut version as *mut *mut c_char as *mut c_void,
        );
        println!("Nix version: {}", CStr::from_ptr(version).to_string_lossy());

        libc::free(version as *mut c_void);
        nix_gc_decref(ptr::null_mut(), value as *mut c_void);
        nix_state_free(state);
        nix_store_free(store);
    }
}
