use std::os::raw::c_char;
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
unsafe extern "C" fn curio_request_get(url: *const c_char) -> *mut curio_t {
  let url_rs = match CStr::from_ptr(url).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut()
  };
  let mut res = match Request::get(url_rs).send() {
    Ok(v) => v,
    Err(_) => return std::ptr::null_mut()
  };
  res.warnings.shrink_to_fit();
  let mut warnings: Vec<*mut c_char> = res.warnings.iter().map(|i| match CString::new(i.as_str()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut()
    } ).collect();
  let curio = Box::new(curio_t{
    raw: match CString::new(res.raw) {
      Ok(v) => v.into_raw(),
      Err(_) => std::ptr::null_mut()
    },
    protocol: match res.protocol {
      Some(v) => {
        match CString::new(v) {
          Ok(v) => v.into_raw(),
          Err(_) => std::ptr::null_mut()
        }
      },
      None => std::ptr::null_mut()
    },
    status: match res.status {
      Some(v) => v,
      None => 0
    },
    status_text: match res.status_text {
      Some(v) => {
        match CString::new(v) {
          Ok(v) => v.into_raw(),
          Err(_) => std::ptr::null_mut()
        }
      },
      None => std::ptr::null_mut()
    },
    headers: Box::into_raw(Box::new(res.headers)) as *mut c_void,
    header_count: res.header_count,
    cookies: Box::into_raw(Box::new(res.cookies)) as *mut c_void,
    cookie_count: res.cookie_count,
    body: match res.body {
      Some(v) => {
        match CString::new(v) {
          Ok(v) => v.into_raw(),
          Err(_) => std::ptr::null_mut()
        }
      },
      None => std::ptr::null_mut()
    },
    warnings: warnings.as_mut_ptr(),
    warnings_count: warnings.len()
  });
  std::mem::forget(warnings);
  Box::into_raw(curio)
}