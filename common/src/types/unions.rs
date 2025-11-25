#![allow(nonstandard_style)]
use crate::{
    UNION,
    types::{structs::_LARGE_INTEGER_PART, types::LONGLONG},
};

UNION! {_LARGE_INTEGER {
    s: _LARGE_INTEGER_PART,
    u: _LARGE_INTEGER_PART,
    QuadPart: LONGLONG,
}}
