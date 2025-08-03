use nix_exprc_sys::{
    nix_gc_decref,
    nix_value_force, EvalState, nix_c_context, Value, nix_get_type, nix_init_int, nix_set_err_msg, nix_alloc_primop, nix_get_int, ValueType, nix_register_primop,
    nix_err,
};
use std::ffi::{c_char, c_void};
use std::{ptr};

// https://github.com/NixOS/nix/blob/master/src/external-api-docs/README.md#writing-a-nix-language-plug-in

#[unsafe(no_mangle)]
pub extern "C" fn increment(
    _user_data: *mut c_void,
    ctx: *mut nix_c_context,
    state: *mut EvalState,
    args: *mut *mut Value,
    v: *mut Value,
) {
    unsafe {
        nix_value_force(ptr::null_mut(), state, *args);
        if nix_get_type(ptr::null_mut(), *args) == ValueType::NIX_TYPE_INT {
            let val = nix_get_int(std::ptr::null_mut(), *args) + 1;
            nix_init_int(std::ptr::null_mut(), v, val);
        } else {
            let msg = std::ffi::CString::new("First argument should be an integer.").unwrap();
            nix_set_err_msg(ctx, nix_err::NIX_ERR_UNKNOWN, msg.as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nix_plugin_entry() {
    unsafe {
        let mut args: [*const c_char; 2] = [
            "n\0".as_ptr() as *const c_char,
            ptr::null(),
        ];
        let p = nix_alloc_primop(
            ptr::null_mut(),
            Some(increment),
            1,
            "increment\0".as_ptr() as *const c_char,
            args.as_mut_ptr(),
            "Example custom built-in function: increments an integer\0".as_ptr() as *const c_char,
            ptr::null_mut(),
        );
        nix_register_primop(ptr::null_mut(), p);
        nix_gc_decref(ptr::null_mut(), p as *const c_void);
    }
}
