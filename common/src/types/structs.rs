#![allow(nonstandard_style)]
use crate::{
    STRUCT,
    types::types::{LONG, LONGLONG, P_LIST_ENTRY, P_WCHAR, ULONG, USHORT},
};

STRUCT! {_QUAD {
    UseThisFieldToCopy: LONGLONG,
}}

STRUCT! {_LIST_ENTRY {
    Flink: P_LIST_ENTRY,
    Blink: P_LIST_ENTRY,
}}

STRUCT! {_UNICODE_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: P_WCHAR,
}}

STRUCT! {_LARGE_INTEGER_PART {
    LowPart: ULONG,
    HighPart: LONG,
}}
