#![allow(nonstandard_style)]
extern crate alloc;

use alloc::alloc::handle_alloc_error;
use core::{
    alloc::{AllocError, Allocator, GlobalAlloc, Layout},
    ptr::NonNull,
};

use crate::{
    include::{
        ExAllocatePool, ExFreePool, MmAllocateContiguousMemorySpecifyCacheNode,
        MmFreeContiguousMemory,
    },
    types::{
        enums::MEMORY_CACHING_TYPE::MmCached,
        types::{BYTE, P_BYTE, P_VOID, PHYSICAL_ADDRESS, SIZE_T},
    },
};

/// The global kernel allocator structure.
#[derive(Clone)]
pub struct KernelAlloc;

/// The physical kernel allocator structure.
#[derive(Clone)]
pub struct PhysicalAllocator;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> P_BYTE {
        let pool = unsafe { ExAllocatePool(0, layout.size() as SIZE_T) };
        if pool.is_null() {
            handle_alloc_error(layout);
        }

        pool as P_BYTE
    }

    unsafe fn dealloc(&self, ptr: P_BYTE, _layout: Layout) {
        unsafe { ExFreePool(ptr as P_VOID) };
    }
}

unsafe impl Allocator for PhysicalAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[BYTE]>, AllocError> {
        let mut boundary: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };
        let mut lowest: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };
        let mut highest: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };

        boundary.QuadPart = 0;
        lowest.QuadPart = 0;
        highest.QuadPart = -1;

        let memory = unsafe {
            MmAllocateContiguousMemorySpecifyCacheNode(
                layout.size() as SIZE_T,
                lowest,
                highest,
                boundary,
                MmCached,
                0x80000000,
            )
        } as P_BYTE;
        if memory.is_null() {
            Err(AllocError)
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(memory, layout.size()) };
            Ok(unsafe { NonNull::new_unchecked(slice) })
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<BYTE>, _layout: Layout) {
        unsafe { MmFreeContiguousMemory(ptr.cast().as_ptr()) };
    }
}

unsafe impl Allocator for KernelAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[BYTE]>, AllocError> {
        let memory = unsafe { ExAllocatePool(0, layout.size() as SIZE_T) } as P_BYTE;
        if memory.is_null() {
            Err(AllocError)
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(memory, layout.size()) };
            Ok(unsafe { NonNull::new_unchecked(slice) })
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<BYTE>, _layout: Layout) {
        unsafe { ExFreePool(ptr.cast::<u64>().as_ptr() as P_VOID) };
    }
}
