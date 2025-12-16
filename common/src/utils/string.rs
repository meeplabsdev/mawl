extern crate alloc;
use alloc::string::String as _String;
use alloc::vec::Vec;

use crate::types_old::types::{P_WCHAR, PC_WCHAR, UNICODE_STRING, USHORT, WCHAR};

pub struct String(_String);

impl String {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for String {
    fn default() -> Self {
        Self(_String::default())
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<&str> for String {
    fn from(p_str: &str) -> Self {
        Self(_String::from(p_str))
    }
}

impl From<PC_WCHAR> for String {
    fn from(pcwstr: PC_WCHAR) -> Self {
        Self(
            unsafe { widestring::U16CStr::from_ptr_str(pcwstr) }
                .to_string()
                .unwrap_or(_String::new()),
        )
    }
}

impl From<UNICODE_STRING> for String {
    fn from(unicode_string: UNICODE_STRING) -> Self {
        Self(
            unsafe {
                widestring::U16CString::from_ptr(
                    unicode_string.Buffer,
                    unicode_string.Length as usize,
                )
            }
            .expect("Invalid unicode string supplied")
            .to_string()
            .unwrap_or(_String::new()),
        )
    }
}

impl Into<_String> for String {
    fn into(self) -> _String {
        self.0
    }
}

impl Into<PC_WCHAR> for String {
    fn into(self) -> PC_WCHAR {
        let wstr: Vec<WCHAR> = widestring::encode_utf16(self.0.chars()).collect();
        wstr.as_ptr()
    }
}

impl Into<UNICODE_STRING> for String {
    fn into(self) -> UNICODE_STRING {
        let wstr: Vec<WCHAR> = widestring::encode_utf16(self.0.chars()).collect();
        UNICODE_STRING {
            Length: wstr.len() as USHORT,
            MaximumLength: wstr.len() as USHORT,
            Buffer: wstr.as_ptr() as P_WCHAR,
        }
    }
}
