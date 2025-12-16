#![allow(nonstandard_style)]
use core::ffi::c_void;

use crate::{
    TYPE,
    types_old::{
        structs::{
            P_DEVICE_OBJECT, P_EPROCESS, P_ERESOURCE, P_IO_STATUS_BLOCK, P_IRP, P_MDL,
            PC_DEVICE_OBJECT, PC_DRIVER_OBJECT, PC_ERESOURCE, PC_FILE_OBJECT, PC_IRP, PC_KDPC,
            PC_MDL, PC_UNICODE_STRING,
        },
        unions::LARGE_INTEGER,
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
TYPE!(NTSTATUS = UINT);
TYPE!(SECURITY_DESCRIPTOR_CONTROL = WORD);

pub type PHYSICAL_ADDRESS = LARGE_INTEGER;
pub type NODE_REQUIREMENT = ULONG;
pub type POOL_TYPE = LONG;
pub type DEVICE_TYPE = ULONG;
pub type KIRQL = UCHAR;
pub type KSPIN_LOCK = ULONG;

TYPE!(DRIVER_INITIALIZE = Option<unsafe extern "system" fn(DeviceObject: PC_DRIVER_OBJECT, RegistryPath: PC_UNICODE_STRING) -> NTSTATUS>);
TYPE!(DRIVER_UNLOAD = Option<unsafe extern "system" fn(DeviceObject: PC_DRIVER_OBJECT)>);
TYPE!(DRIVER_DISPATCH = Option<unsafe extern "system" fn(DeviceObject: PC_DEVICE_OBJECT, Irp: P_IRP) -> NTSTATUS>);
TYPE!(DRIVER_CANCEL = Option<unsafe extern "system" fn(DeviceObject: P_DEVICE_OBJECT, Irp: P_IRP)>);
TYPE!(DRIVER_CONTROL = Option<unsafe extern "system" fn(DeviceObject: P_DEVICE_OBJECT, Irp: P_IRP, MapRegisterBase: P_VOID, Context: P_VOID) -> IO_ALLOCATION_ACTION>);

TYPE!(FS_FILTER_CALLBACK = Option<unsafe extern "system" fn(data: PC_FS_FILTER_CALLBACK_DATA, completioncontext: *mut P_VOID) -> NTSTATUS>);
TYPE!(FS_FILTER_COMPLETION_CALLBACK = Option<unsafe extern "system" fn(data: PC_FS_FILTER_CALLBACK_DATA, operationstatus: NTSTATUS, completioncontext: PC_VOID)>);

TYPE!(FAST_IO_CHECK_IF_POSSIBLE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, wait: BOOL, lockkey: ULONG, checkforreadoperation: BOOL, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_READ = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, wait: BOOL, lockkey: ULONG, buffer: P_VOID, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_WRITE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, wait: BOOL, lockkey: ULONG, buffer: PC_VOID, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_QUERY_BASIC_INFO = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, wait: BOOL, buffer: P_FILE_BASIC_INFORMATION, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_QUERY_STANDARD_INFO = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, wait: BOOL, buffer: P_FILE_STANDARD_INFORMATION, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_LOCK = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: PC_LONGLONG, processid: P_EPROCESS, key: ULONG, failimmediately: BOOL, exclusivelock: BOOL, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_UNLOCK_SINGLE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: PC_LONGLONG, processid: P_EPROCESS, key: ULONG, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_UNLOCK_ALL = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, processid: P_EPROCESS, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_UNLOCK_ALL_BY_KEY = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, processid: PC_VOID, key: ULONG, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_DEVICE_CONTROL = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, wait: BOOL, inputbuffer: PC_VOID, inputbufferlength: ULONG, outputbuffer: P_VOID, outputbufferlength: ULONG, iocontrolcode: ULONG, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_ACQUIRE_FILE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT)>);
TYPE!(FAST_IO_RELEASE_FILE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT)>);
TYPE!(FAST_IO_DETACH_DEVICE = Option<unsafe extern "system" fn(sourcedevice: PC_DEVICE_OBJECT, targetdevice: PC_DEVICE_OBJECT)>);
TYPE!(FAST_IO_QUERY_NETWORK_OPEN_INFO = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, wait: BOOL, buffer: P_FILE_NETWORK_OPEN_INFORMATION, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_ACQUIRE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, endingoffset: PC_LONGLONG, resourcetorelease: *mut P_ERESOURCE, deviceobject: PC_DEVICE_OBJECT) -> NTSTATUS>);
TYPE!(FAST_IO_MDL_READ = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, lockkey: ULONG, mdlchain: *mut P_MDL, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_MDL_READ_COMPLETE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, mdlchain: PC_MDL, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_PREPARE_MDL_WRITE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, lockkey: ULONG, mdlchain: *mut P_MDL, iostatus: P_IO_STATUS_BLOCK, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_MDL_WRITE_COMPLETE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, mdlchain: P_MDL, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_READ_COMPRESSED = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, lockkey: ULONG, buffer: P_VOID, mdlchain: *mut P_MDL, iostatus: P_IO_STATUS_BLOCK, compresseddatainfo: P_COMPRESSED_DATA_INFO, compresseddatainfolength: ULONG, deviceobject: PC_DEVICE_OBJECT) -> bool>);
TYPE!(FAST_IO_WRITE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, length: ULONG, lockkey: ULONG, buffer: PC_VOID, mdlchain: *mut P_MDL, iostatus: P_IO_STATUS_BLOCK, compresseddatainfo: PC_COMPRESSED_DATA_INFO, compresseddatainfolength: ULONG, deviceobject: PC_DEVICE_OBJECT) -> bool>);
TYPE!(FAST_IO_MDL_READ_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, mdlchain: PC_MDL, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, fileoffset: PC_LONGLONG, mdlchain: PC_MDL, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_QUERY_OPEN = Option<unsafe extern "system" fn(irp: P_IRP, networkinformation: P_FILE_NETWORK_OPEN_INFORMATION, deviceobject: PC_DEVICE_OBJECT) -> BOOL>);
TYPE!(FAST_IO_RELEASE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, resourcetorelease: PC_ERESOURCE, deviceobject: PC_DEVICE_OBJECT) -> NTSTATUS>);
TYPE!(FAST_IO_ACQUIRE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, deviceobject: PC_DEVICE_OBJECT) -> NTSTATUS>);
TYPE!(FAST_IO_RELEASE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: PC_FILE_OBJECT, deviceobject: PC_DEVICE_OBJECT) -> NTSTATUS>);

TYPE!(KDEFERRED_ROUTINE = Option<unsafe extern "system" fn(dpc: PC_KDPC, deferredcontext: PC_VOID, systemargument1: PC_VOID, systemargument2: PC_VOID)>);
TYPE!(EXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: P_EXCEPTION_RECORD, establisherframe: PC_VOID, contextrecord: P_CONTEXT, dispatchercontext: PC_VOID) -> EXCEPTION_DISPOSITION>);
TYPE!(PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>);
TYPE!(IO_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(DeviceObject: PC_DEVICE_OBJECT, Irp: PC_IRP, Context: PC_VOID) -> NTSTATUS>);
TYPE!(IO_APC_ROUTINE = Option<unsafe extern "system" fn(ApcCOntext: P_VOID, IoStatusBlock: P_IO_STATUS_BLOCK, Reserved: ULONG)>);
