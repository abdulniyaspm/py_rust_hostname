use std::ffi::CStr;
use std::io::Error;

use libc;
use pyo3::exceptions;
use pyo3::prelude::*;

const HOSTNAME_BUFFER_SIZE: usize = 1024;

/// Return the name of the localhost
#[pyfunction]
fn get_hostname() -> PyResult<String> {
    let mut buffer: Vec<i8> = Vec::with_capacity(HOSTNAME_BUFFER_SIZE);
    let buffer_ptr = buffer.as_mut_ptr();
    unsafe {
        let returncode = libc::gethostname(
            buffer_ptr as *mut libc::c_char,
            HOSTNAME_BUFFER_SIZE as libc::size_t,
        );
        if returncode < 0 {
            return Err(exceptions::PyOSError::new_err(
                Error::last_os_error().to_string(),
            ));
        } else {
            let cstr = CStr::from_ptr(buffer_ptr);
            return Ok(cstr.to_str().unwrap().to_owned());
        }
    }
}

#[pymodule]
fn py_rust_hostname(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_hostname, m)?)?;

    Ok(())
}
