use std::{ffi, mem, ptr};
use helpers::*;

use winapi::ctypes::c_void;
use winapi::shared::ntdef::HRESULT;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::guiddef::REFIID;
use winapi::um::d2d1::*;
use winapi::um::libloaderapi::*;

type CreateFactory = extern "system" fn(
    D2D1_FACTORY_TYPE,
    REFIID,
    *const D2D1_FACTORY_OPTIONS,
    *mut *mut c_void,
) -> HRESULT;

#[derive(Debug, PartialEq)]
pub struct D2D1 {
    handle: HMODULE,
}

impl D2D1 {
    pub fn load() -> Result<D2D1, HRESULT> {
        unsafe {
            let dll = ffi::CString::new("D2D1.dll").unwrap();
            let mut handle = GetModuleHandleA(dll.as_ptr());
            if handle == ptr::null_mut() {
                handle = LoadLibraryA(dll.as_ptr());
            }

            if handle != ptr::null_mut() {
                Ok(D2D1 { handle: handle })
            } else {
                Err(last_error_hr())
            }
        }
    }

    pub unsafe fn create_factory(
        &self,
        ftype: D2D1_FACTORY_TYPE,
        riid: REFIID,
        options: *const D2D1_FACTORY_OPTIONS,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        let procedure = ffi::CString::new("D2D1CreateFactory").unwrap();
        let create_factory_ptr = GetProcAddress(self.handle, procedure.as_ptr());

        if create_factory_ptr == ptr::null_mut() {
            panic!(
                "Error loading function D2D1CreateFactory: {:?}",
                last_error_string()
            );
        }

        let create_factory: CreateFactory = mem::transmute(create_factory_ptr);
        create_factory(ftype, riid, options, ppv)
    }
}
