use crate::error::Error;
use crate::ffi::FFIResult;
use libc::c_void;
use std::{ffi::CStr, os::raw::c_char};

use crate::tesseract::Tesseract;

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_new() -> *mut Tesseract {
    Box::into_raw(Box::new(Tesseract::default())) as *mut Tesseract
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_from_file(file: *const c_char) -> FFIResult<Tesseract> {
    if file.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    let cname = CStr::from_ptr(file).to_string_lossy().to_string();
    match Tesseract::from_file(cname) {
        Ok(tesseract) => FFIResult::ok(tesseract),
        Err(e) => FFIResult::err(e),
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_to_file(
    tesseract: *mut Tesseract,
    file: *const c_char,
) -> FFIResult<c_void> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    if file.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Key is null")));
    }
    let tesseract = &mut *tesseract;
    let cname = CStr::from_ptr(file).to_string_lossy().to_string();
    FFIResult::from(tesseract.to_file(cname))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_set_file(
    tesseract: *mut Tesseract,
    file: *const c_char,
) -> bool {
    if tesseract.is_null() {
        return false;
    }

    if file.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    let cname = CStr::from_ptr(file).to_string_lossy().to_string();
    tesseract.set_file(cname);
    true
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_set_autosave(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    tesseract.set_autosave();
    true
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_autosave_enabled(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &*tesseract;
    tesseract.autosave_enabled()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_disable_key_check(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    tesseract.disable_key_check();
    true
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_enable_key_check(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    tesseract.enable_key_check();
    true
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_is_key_check_enabled(tesseract: *const Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &*tesseract;
    tesseract.is_key_check_enabled()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_save(tesseract: *mut Tesseract) -> FFIResult<c_void> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    let tesseract = &mut *tesseract;
    FFIResult::from(tesseract.save())
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_set(
    tesseract: *mut Tesseract,
    key: *const c_char,
    val: *const c_char,
) -> FFIResult<c_void> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }
    if key.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Key is null")));
    }
    if val.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Val is null")));
    }

    let tesseract = &mut *tesseract;
    let c_key = CStr::from_ptr(key).to_string_lossy().to_string();
    let c_val = CStr::from_ptr(val).to_string_lossy().to_string();

    FFIResult::from(tesseract.set(&c_key, &c_val))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_retrieve(
    tesseract: *mut Tesseract,
    key: *const c_char,
) -> FFIResult<c_char> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    if key.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Key is null")));
    }

    let tesseract = &mut *tesseract;
    let c_key = CStr::from_ptr(key).to_string_lossy().to_string();

    FFIResult::from(tesseract.retrieve(&c_key))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_exist(tesseract: *mut Tesseract, key: *const c_char) -> bool {
    if tesseract.is_null() {
        return false;
    }
    if key.is_null() {
        return false;
    }

    let tesseract = &*tesseract;
    let c_key = CStr::from_ptr(key).to_string_lossy().to_string();
    tesseract.exist(&c_key)
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_delete(
    tesseract: *mut Tesseract,
    key: *const c_char,
) -> FFIResult<c_void> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    if key.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Key is null")));
    }

    let tesseract = &mut *tesseract;
    let c_key = CStr::from_ptr(key).to_string_lossy().to_string();
    FFIResult::from(tesseract.delete(&c_key))
}
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_clear(tesseract: *mut Tesseract) {
    if tesseract.is_null() {
        return;
    }

    let tesseract = &mut *tesseract;
    tesseract.clear()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_is_unlock(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    tesseract.is_unlock()
}

//TODO: Have key be bytes
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_unlock(
    tesseract: *mut Tesseract,
    key: *const c_char,
) -> FFIResult<c_void> {
    if tesseract.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Tesseract is null")));
    }

    if key.is_null() {
        return FFIResult::err(Error::Any(anyhow::anyhow!("Key is null")));
    }

    let tesseract = &mut *tesseract;
    let c_key = CStr::from_ptr(key).to_string_lossy().to_string();
    FFIResult::from(tesseract.unlock(c_key.as_bytes()))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn tesseract_lock(tesseract: *mut Tesseract) -> bool {
    if tesseract.is_null() {
        return false;
    }

    let tesseract = &mut *tesseract;
    tesseract.lock();
    true
}
