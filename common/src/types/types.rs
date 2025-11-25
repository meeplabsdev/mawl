#![allow(nonstandard_style)]
use core::ffi::c_void;

use crate::{
    TYPE,
    types::{
        structs::{_LIST_ENTRY, _QUAD, _UNICODE_STRING},
        unions::_LARGE_INTEGER,
    },
};

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type wchar_t = u16;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;
pub type c_bool = bool;
pub type __int8 = i8;
pub type __uint8 = u8;
pub type __int16 = i16;
pub type __uint16 = u16;
pub type __int32 = i32;
pub type __uint32 = u32;
pub type __int64 = i64;
pub type __uint64 = u64;

TYPE!(VOID = c_void);
TYPE!(CHAR = c_char);
TYPE!(SCHAR = c_schar);
TYPE!(UCHAR = c_uchar);
TYPE!(WCHAR = wchar_t);
TYPE!(SHORT = c_short);
TYPE!(USHORT = c_ushort);
TYPE!(INT = c_int);
TYPE!(UINT = c_uint);
TYPE!(LONG = c_long);
TYPE!(ULONG = c_ulong);
TYPE!(LONGLONG = c_longlong);
TYPE!(ULONGLONG = c_ulonglong);
TYPE!(FLOAT = c_float);
TYPE!(DOUBLE = c_double);
TYPE!(BOOL = c_bool);

TYPE!(ATOM = WORD);
TYPE!(BOOLEAN = BYTE);
TYPE!(BYTE = UCHAR);
TYPE!(DWORD = ULONG);
TYPE!(DWORDLONG = ULONGLONG);
TYPE!(WORD = USHORT);
TYPE!(HANDLE = P_VOID);
TYPE!(SIZE_T = LONGLONG);

TYPE!(QUAD = _QUAD);
TYPE!(LIST_ENTRY = _LIST_ENTRY);
TYPE!(UNICODE_STRING = _UNICODE_STRING);
TYPE!(LARGE_INTEGER = _LARGE_INTEGER);

pub type PHYSICAL_ADDRESS = LARGE_INTEGER;
pub type NODE_REQUIREMENT = UINT;
pub type POOL_TYPE = LONG;
