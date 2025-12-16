#![allow(nonstandard_style)]

use crate::ENUM;

ENUM! {MEMORY_CACHING_TYPE {
    MmNonCached = 0,
    MmCached = 1,
    MmWriteCombined = 2,
    MmHardwareCoherentCached = 3,
    MmNonCachedUnordered = 4,
    MmUSWCCached = 5,
    MmMaximumCacheType = 6,
    MmNotMapped = -1,
}}

ENUM! {DEVICE_POWER_STATE {
    PowerDeviceUnspecified,
    PowerDeviceD0,
    PowerDeviceD1,
    PowerDeviceD2,
    PowerDeviceD3,
    PowerDeviceMaximum,
}}

ENUM! {KPROCESSOR_MODE {
    KernelMode,
    UserMode,
}}

ENUM! {IOP_PRIORITY_HINT {
    IopIoPriorityNotSet = 0,
    IopIoPriorityVeryLow = 1,
    IopIoPriorityLow = 2,
    IopIoPriorityNormal = 3,
    IopIoPriorityHigh = 4,
    IopIoPriorityCritical = 5,
    MaxIopIoPriorityTypes = 6
}}

ENUM! {PS_QUOTA_TYPE {
    PsNonPagedPool = 0,
    PsPagedPool,
    PsPageFile,
    PsWorkingSet,
    PsQuotaTypes,
}}
