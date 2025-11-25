#![allow(nonstandard_style)]
use crate::types::{
    enums::MEMORY_CACHING_TYPE,
    types::{NODE_REQUIREMENT, P_VOID, PC_CHAR, PHYSICAL_ADDRESS, POOL_TYPE, SIZE_T, ULONG, VOID},
};

#[link(name = "ntoskrnl")]
unsafe extern "system" {
    #[cfg(debug_assertions)]
    pub(crate) unsafe fn DbgPrintEx(ComponentId: ULONG, Level: ULONG, Format: PC_CHAR) -> ULONG;

    pub(crate) unsafe fn ExFreePool(P: P_VOID) -> VOID;
    pub(crate) unsafe fn ExAllocatePool(PoolType: POOL_TYPE, NumberOfBytes: SIZE_T) -> P_VOID;
    pub(crate) unsafe fn MmFreeContiguousMemory(BaseAddress: P_VOID) -> VOID;
    pub(crate) unsafe fn MmAllocateContiguousMemorySpecifyCacheNode(
        NumberOfBytes: SIZE_T,
        LowestAcceptableAddress: PHYSICAL_ADDRESS,
        HighestAcceptableAddress: PHYSICAL_ADDRESS,
        BoundaryAddressMultiple: PHYSICAL_ADDRESS,
        CacheType: MEMORY_CACHING_TYPE,
        PreferredNode: NODE_REQUIREMENT,
    ) -> P_VOID;
}

// unsafe extern "system" {
//     pub(crate) unsafe fn NtShutdownSystem(Action: u16) -> !;
// }
