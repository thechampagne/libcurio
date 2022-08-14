use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use curio::structs::Request;

#[repr(C)]
struct curio_t {
    raw: *mut c_char,
    protocol: *mut c_char,
    status: isize,
    status_text: *mut c_char,
    headers: *mut c_void,
    header_count: usize,
    cookies: *mut c_void,
    cookie_count: usize,
    body: *mut c_char,
    warnings: *mut *mut c_char,
    warnings_count: usize
}

#[no_mangle]
unsafe extern "C" fn curio_request_get(curio: *mut curio_t, url: *const c_char) -> c_int {
  let url_rs = match CStr::from_ptr(url).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
  };
  let mut res = match Request::get(url_rs).send() {
    Ok(v) => v,
    Err(_) => return -1
  };
  (*curio).raw = match CString::new(res.raw) {
    Ok(v) => v.into_raw(),
    Err(_) => std::ptr::null_mut()
  };
  (*curio).protocol = match res.protocol {
    Some(v) => {
      match CString::new(v) {
        Ok(v) => v.into_raw(),
        Err(_) => std::ptr::null_mut()
      }
    },
    None => std::ptr::null_mut()
  };
  (*curio).status = match res.status {
    Some(v) => v,
    None => 0
  };
  (*curio).status_text = match res.status_text {
    Some(v) => {
      match CString::new(v) {
        Ok(v) => v.into_raw(),
        Err(_) => std::ptr::null_mut()
      }
    },
    None => std::ptr::null_mut()
  };
  (*curio).headers = Box::into_raw(Box::new(res.headers)) as *mut c_void;
  (*curio).header_count = res.header_count;
  (*curio).cookies = Box::into_raw(Box::new(res.cookies)) as *mut c_void;
  (*curio).cookie_count = res.cookie_count;
  (*curio).body = match res.body {
    Some(v) => {
      match CString::new(v) {
        Ok(v) => v.into_raw(),
        Err(_) => std::ptr::null_mut()
      }
    },
    None => std::ptr::null_mut()
  };
  res.warnings.shrink_to_fit();
  let mut warnings: Vec<*mut c_char> = res.warnings.iter().map(|i| match CString::new(i.as_str()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut()
    } ).collect();
  (*curio).warnings = warnings.as_mut_ptr();
  (*curio).warnings_count = warnings.len();
  std::mem::forget(warnings);
  0
}