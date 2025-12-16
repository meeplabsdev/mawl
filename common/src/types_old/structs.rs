#![allow(nonstandard_style)]
use crate::{
    STRUCT,
    types_old::{
        enums::{
            DEVICE_POWER_STATE, IOP_PRIORITY_HINT, KPROCESSOR_MODE, PS_QUOTA_TYPE::PsQuotaTypes,
        },
        types::{
            BOOLEAN, BYTE, CHAR, DEVICE_TYPE, EXCEPTION_ROUTINE, HANDLE, KIRQL, KSPIN_LOCK, LONG,
            LONGLONG, NTSTATUS, P_CHAR, P_DRIVER_CANCEL, P_DRIVER_CONTROL, P_DRIVER_DISPATCH,
            P_DRIVER_INITIALIZE, P_DRIVER_UNLOAD, P_FAST_IO_ACQUIRE_FILE,
            P_FAST_IO_ACQUIRE_FOR_CCFLUSH, P_FAST_IO_ACQUIRE_FOR_MOD_WRITE,
            P_FAST_IO_CHECK_IF_POSSIBLE, P_FAST_IO_DETACH_DEVICE, P_FAST_IO_DEVICE_CONTROL,
            P_FAST_IO_LOCK, P_FAST_IO_MDL_READ, P_FAST_IO_MDL_READ_COMPLETE,
            P_FAST_IO_MDL_READ_COMPLETE_COMPRESSED, P_FAST_IO_MDL_WRITE_COMPLETE,
            P_FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED, P_FAST_IO_PREPARE_MDL_WRITE,
            P_FAST_IO_QUERY_BASIC_INFO, P_FAST_IO_QUERY_NETWORK_OPEN_INFO, P_FAST_IO_QUERY_OPEN,
            P_FAST_IO_QUERY_STANDARD_INFO, P_FAST_IO_READ, P_FAST_IO_READ_COMPRESSED,
            P_FAST_IO_RELEASE_FILE, P_FAST_IO_RELEASE_FOR_CCFLUSH, P_FAST_IO_RELEASE_FOR_MOD_WRITE,
            P_FAST_IO_UNLOCK_ALL, P_FAST_IO_UNLOCK_ALL_BY_KEY, P_FAST_IO_UNLOCK_SINGLE,
            P_FAST_IO_WRITE, P_FAST_IO_WRITE_COMPRESSED, P_FS_FILTER_CALLBACK,
            P_FS_FILTER_COMPLETION_CALLBACK, P_HANDLE, P_IO_COMPLETION_ROUTINE,
            P_KDEFERRED_ROUTINE, P_LONG, P_VOID, P_WCHAR, PPS_POST_PROCESS_INIT_ROUTINE,
            SECURITY_DESCRIPTOR_CONTROL, SHORT, SIZE_T, UCHAR, ULONG, ULONGLONG, USHORT, WCHAR,
            WORD,
        },
        unions::{
            ASSOCIATED_IRP, DISPATCHER_HEADER_1, DISPATCHER_HEADER_1_1_1, DISPATCHER_HEADER_1_1_2,
            DISPATCHER_HEADER_1_1_3, EPROCESS_1, EPROCESS_2, EPROCESS_3, ERESOURCE_1, ETHREAD_1,
            ETHREAD_2, ETHREAD_3, ETHREAD_4, ETHREAD_5, ETHREAD_6, EX_FAST_REF_1, EX_PUSH_LOCK_1,
            EX_RUNDOWN_REF_1, IO_STACK_LOCATION_1, IO_STATUS_BLOCK_1, IRP_1, KPROCESS_1,
            KPROCESS_2, KTHREAD_1, KTHREAD_2, KTHREAD_3, KTHREAD_4, KTHREAD_5, KTHREAD_6,
            KTHREAD_7, KTHREAD_8, KTHREAD_9, KTHREAD_10, KTHREAD_11, KTHREAD_12, LARGE_INTEGER,
            OVERLAY_IRP, OVERLAY_IRP_2, OVERLAY_IRP_3, OWNER_ENTRY_1, PS_CLIENT_SECURITY_CONTEXT_1,
            PSP_RATE_APC_1, QUEUE, TAIL_IRP, TAIL_OVERLAY_IRP_1, TAIL_OVERLAY_IRP_4,
            ULARGE_INTEGER, WAIT_CONTEXT_BLOCK_1,
        },
    },
};

STRUCT! {QUAD {
    UseThisFieldToCopy: LONGLONG,
}}

STRUCT! {LIST_ENTRY {
    Flink: P_LIST_ENTRY,
    Blink: P_LIST_ENTRY,
}}

STRUCT! {SINGLE_LIST_ENTRY {
    Next: P_SINGLE_LIST_ENTRY,
}}

STRUCT! {UNICODE_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: P_WCHAR,
}}

STRUCT! {LARGE_INTEGER_PART {
    LowPart: ULONG,
    HighPart: LONG,
}}

STRUCT! {ULARGE_INTEGER_PART {
    LowPart: ULONG,
    HighPart: ULONG,
}}

STRUCT! {DRIVER_OBJECT {
    Type: SHORT,
    Size: SHORT,
    DeviceObject: P_DEVICE_OBJECT,
    Flags: ULONG,
    DriverStart: P_VOID,
    DriverSize: ULONG,
    DriverSection: P_VOID,
    DriverExtension: P_DRIVER_EXTENSION,
    DriverName: UNICODE_STRING,
    FastIoDispatch: P_FAST_IO_DISPATCH,
    DriverInit: P_DRIVER_INITIALIZE,
    DriverStartIo: P_DRIVER_UNLOAD,
    MajorFunction: P_DRIVER_DISPATCH,
}}

STRUCT! {DEVICE_OBJECT {
    Type: SHORT,
    Size: USHORT,
    ReferenceCount: LONG,
    DriverObject: P_DRIVER_OBJECT,
    NextDevice: P_DEVICE_OBJECT,
    AttachedDevice: P_DEVICE_OBJECT,
    CurrentIrp: P_IRP,
    Timer: P_IO_TIMER,
    Flags: ULONG,
    Characteristics: ULONG,
    Vpb: P_VPB,
    DeviceExtension: P_VOID,
    DeviceType: DEVICE_TYPE,
    StackSize: CHAR,
    Queue: QUEUE,
    AlignmentRequirement: ULONG,
    DeviceQueue: KDEVICE_QUEUE,
    Dpc: KDPC,
    ActiveThreadCount: ULONG,
    SecurityDescriptor: P_SECURITY_DESCRIPTOR,
    DeviceLock: KEVENT,
    SectorSize: USHORT,
    Spare1: USHORT,
    DeviceObjectExtension: P_DEVOBJ_EXTENSION,
    Reserved: P_VOID,
}}

STRUCT! {DRIVER_EXTENSION {
    DriverObject: P_DRIVER_OBJECT,
    AddDevice: P_LONG,
    Count: ULONG,
    ServiceKeyName: UNICODE_STRING,
    ClientDriverExtension: P_IO_CLIENT_EXTENSION,
    FsFilterCallbacks: P_FS_FILTER_CALLBACKS,
}}

STRUCT! {FAST_IO_DISPATCH{
    SizeOfFastIoDispatch: ULONG,
    FastIoCheckIfPossible: P_FAST_IO_CHECK_IF_POSSIBLE,
    FastIoRead: P_FAST_IO_READ,
    FastIoWrite: P_FAST_IO_WRITE,
    FastIoQueryBasicInfo: P_FAST_IO_QUERY_BASIC_INFO,
    FastIoQueryStandardInfo: P_FAST_IO_QUERY_STANDARD_INFO,
    FastIoLock: P_FAST_IO_LOCK,
    FastIoUnlockSingle: P_FAST_IO_UNLOCK_SINGLE,
    FastIoUnlockAll: P_FAST_IO_UNLOCK_ALL,
    FastIoUnlockAllByKey: P_FAST_IO_UNLOCK_ALL_BY_KEY,
    FastIoDeviceControl: P_FAST_IO_DEVICE_CONTROL,
    AcquireFileForNtCreateSection: P_FAST_IO_ACQUIRE_FILE,
    ReleaseFileForNtCreateSection: P_FAST_IO_RELEASE_FILE,
    FastIoDetachDevice: P_FAST_IO_DETACH_DEVICE,
    FastIoQueryNetworkOpenInfo: P_FAST_IO_QUERY_NETWORK_OPEN_INFO,
    AcquireForModWrite: P_FAST_IO_ACQUIRE_FOR_MOD_WRITE,
    MdlRead: P_FAST_IO_MDL_READ,
    MdlReadComplete: P_FAST_IO_MDL_READ_COMPLETE,
    PrepareMdlWrite: P_FAST_IO_PREPARE_MDL_WRITE,
    MdlWriteComplete: P_FAST_IO_MDL_WRITE_COMPLETE,
    FastIoReadCompressed: P_FAST_IO_READ_COMPRESSED,
    FastIoWriteCompressed: P_FAST_IO_WRITE_COMPRESSED,
    MdlReadCompleteCompressed: P_FAST_IO_MDL_READ_COMPLETE_COMPRESSED,
    MdlWriteCompleteCompressed: P_FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED,
    FastIoQueryOpen: P_FAST_IO_QUERY_OPEN,
    ReleaseForModWrite: P_FAST_IO_RELEASE_FOR_MOD_WRITE,
    AcquireForCcFlush: P_FAST_IO_ACQUIRE_FOR_CCFLUSH,
    ReleaseForCcFlush: P_FAST_IO_RELEASE_FOR_CCFLUSH,
}}

STRUCT! {IRP {
    Type: SHORT,
    Size: USHORT,
    MdlAddress: P_MDL,
    Flags: ULONG,
    AssociatedIrp: ASSOCIATED_IRP,
    ThreadListEntry: LIST_ENTRY,
    IoStatus: IO_STATUS_BLOCK,
    RequestorMode: KPROCESSOR_MODE,
    PendingReturned: BOOLEAN,
    StackCount: CHAR,
    CurrentLocation: CHAR,
    Cancel: BOOLEAN,
    CancelIrql: KIRQL,
    ApcEnvironment: CHAR,
    AllocationFlags: UCHAR,
    Anonymous: IRP_1,
    UserEvent: P_KEVENT,
    Overlay: OVERLAY_IRP,
    CancelRoutine: P_DRIVER_CANCEL,
    UserBuffer: P_VOID,
    Tail: TAIL_IRP,
}}

STRUCT! {TAIL_OVERLAY_IRP {
    Anonymous: TAIL_OVERLAY_IRP_1,
    Thread: P_ETHREAD,
    AuxiliaryBuffer: P_CHAR,
    Anonymous_1: TAIL_OVERLAY_IRP_2,
    OriginalFileObject: P_FILE_OBJECT,
}}

STRUCT! {TAIL_OVERLAY_IRP_2 {
    ListEntry: LIST_ENTRY,
    Anonymous: TAIL_OVERLAY_IRP_4,
}}

STRUCT! {TAIL_OVERLAY_IRP_3 {
    DriverContext: [P_VOID; 4],
}}

STRUCT! {OVERLAY_IRP_1 {
    Anonymous: OVERLAY_IRP_2,
    Anonymous_1: OVERLAY_IRP_3,
}}

STRUCT! {IO_TIMER {
    Type: SHORT,
    TimerFlag: SHORT,
    TimerList: LIST_ENTRY,
    TimerRoutine: P_VOID,
    Context: P_VOID,
    DeviceObject: P_DEVICE_OBJECT,
}}

STRUCT! {VPB {
    Type: SHORT,
    Size: SHORT,
    Flags: USHORT,
    VolumeLabelLength: USHORT,
    DeviceObject: P_DEVICE_OBJECT,
    RealDevice: P_DEVICE_OBJECT,
    SerialNumber: ULONG,
    ReferenceCount: ULONG,
    VolumeLabel: WCHAR,
}}

STRUCT! {KDEVICE_QUEUE {
    Type: SHORT,
    Size: SHORT,
    DeviceListHead: LIST_ENTRY,
    Lock: ULONG,
    Busy: UCHAR,
}}

STRUCT! {KDPC {
    Anonymous: KDPC_1,
    DpcListEntry: SINGLE_LIST_ENTRY,
    ProcessorHistory: UCHAR,
    DeferredRoutine: P_KDEFERRED_ROUTINE,
    DeferredContext: P_VOID,
    SystemArgument1: P_VOID,
    SystemArgument2: P_VOID,
    DpcData: P_VOID,
}}

STRUCT! {KDPC_1 {
    TargetInfoAsUlong: ULONG,
    Anonymous: KDPC_2,
}}

STRUCT! {KDPC_2 {
    Type: UCHAR,
    Importance: UCHAR,
    Number: USHORT,
}}

STRUCT! {SECURITY_DESCRIPTOR {
    Revision: BYTE,
    Sbz1: BYTE,
    Control: SECURITY_DESCRIPTOR_CONTROL,
    Owner: P_SID,
    Group: P_SID,
    Sacl: P_ACL,
    Dacl: P_ACL,
}}

STRUCT! {SID {
    Revision: UCHAR,
    SubAuthorityCount: UCHAR,
    IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    SubAuthority: [ULONG; 1],
}}

STRUCT! {ACL {
    AclRevision: UCHAR,
    Sbz1: UCHAR,
    AclSize: USHORT,
    AceCount: USHORT,
    Sbz2: USHORT,
}}

STRUCT! {KEVENT {
    Header: DISPATCHER_HEADER,
}}

STRUCT! {DEVOBJ_EXTENSION {
    Type: SHORT,
    Size: WORD,
    DeviceObject: P_DEVICE_OBJECT,
    PowerFlags: ULONG,
    Dope: P_DEVICE_OBJECT_POWER_EXTENSION,
    ExtensionFlags: ULONG,
    DeviceNode: P_VOID,
    AttachedTo: P_DEVICE_OBJECT,
    StartIoCount: LONG,
    StartIoKey: LONG,
    StartIoFlags: ULONG,
    Vpb: P_VPB,
}}

STRUCT! {DEVICE_OBJECT_POWER_EXTENSION {
    IdleCount: LONG,
    ConservationIdleTime: ULONG,
    PerformanceIdleTime: ULONG,
    DeviceObject: P_DEVICE_OBJECT,
    IdleList: LIST_ENTRY,
    DeviceType: UCHAR,
    State: DEVICE_POWER_STATE,
    NotifySourceList: LIST_ENTRY,
    NotifyTargetList: LIST_ENTRY,
    PowerChannelSummary: POWER_CHANNEL_SUMMARY,
    Volume: LIST_ENTRY,
    PreviousIdleCount: ULONG,
}}

STRUCT! {POWER_CHANNEL_SUMMARY {
    Signature: ULONG,
    TotalCount: ULONG,
    D0Count: ULONG,
    NotifyList: LIST_ENTRY,
}}

STRUCT! {IO_CLIENT_EXTENSION {
    NextExtension: P_IO_CLIENT_EXTENSION,
    ClientIdentificationAddress: P_VOID,
}}

STRUCT! {FS_FILTER_CALLBACKS {
    SizeOfFsFilterCallbacks: ULONG,
    Reserved: ULONG,
    PreAcquireForSectionSynchronization: P_FS_FILTER_CALLBACK,
    PostAcquireForSectionSynchronization: P_FS_FILTER_COMPLETION_CALLBACK,
    PreReleaseForSectionSynchronization: P_FS_FILTER_CALLBACK,
    PostReleaseForSectionSynchronization: P_FS_FILTER_COMPLETION_CALLBACK,
    PreAcquireForCcFlush: P_FS_FILTER_CALLBACK,
    PostAcquireForCcFlush: P_FS_FILTER_COMPLETION_CALLBACK,
    PreReleaseForCcFlush: P_FS_FILTER_CALLBACK,
    PostReleaseForCcFlush: P_FS_FILTER_COMPLETION_CALLBACK,
    PreAcquireForModifiedPageWriter: P_FS_FILTER_CALLBACK,
    PostAcquireForModifiedPageWriter: P_FS_FILTER_COMPLETION_CALLBACK,
    PreReleaseForModifiedPageWriter: P_FS_FILTER_CALLBACK,
    PostReleaseForModifiedPageWriter: P_FS_FILTER_COMPLETION_CALLBACK,
    PreQueryOpen: P_FS_FILTER_CALLBACK,
    PostQueryOpen: P_FS_FILTER_COMPLETION_CALLBACK,
}}

STRUCT! {MDL {
    Next: P_MDL,
    Size: SHORT,
    MdlFlags: SHORT,
    Process: P_EPROCESS,
    MappedSystemVa: P_VOID,
    StartVa: P_VOID,
    ByteCount: ULONG,
    ByteOffset: ULONG,
}}

STRUCT! {IO_STATUS_BLOCK {
    Anonymous: IO_STATUS_BLOCK_1,
    Information: ULONG,
}}

STRUCT! {ETHREAD {
    Tcb: KTHREAD,
    CreateTime: LARGE_INTEGER,
    Anonymous: ETHREAD_1,
    Anonymous_1: ETHREAD_2,
    Anonymous_2: ETHREAD_3,
    Anonymous_3: ETHREAD_4,
    ActiveTimerListLock: ULONG,
    ActiveTimerListHead: LIST_ENTRY,
    Cid: CLIENT_ID,
    Anonymous_4: ETHREAD_5,
    ClientSecurity: PS_CLIENT_SECURITY_CONTEXT,
    IrpList: LIST_ENTRY,
    TopLevelIrp: ULONG,
    DeviceToVerify: P_DEVICE_OBJECT,
    RateControlApc: P_PSP_RATE_APC,
    Win32StartAddress: P_VOID,
    SparePtr0: P_VOID,
    ThreadListEntry: LIST_ENTRY,
    RundownProtect: EX_RUNDOWN_REF,
    ThreadLock: EX_PUSH_LOCK,
    ReadClusterSize: ULONG,
    MmLockOrdering: LONG,
    CrossThreadFlags: ULONG,
    Flags_0: ULONG,
    SameThreadApcFlags: ULONG,
    Flags_1: ULONG,
    CacheManagerActive: UCHAR,
    DisablePageFaultClustering: UCHAR,
    ActiveFaultCount: UCHAR,
    AlpcMessageId: ULONG,
    Anonymous_5: ETHREAD_6,
    AlpcWaitListEntry: LIST_ENTRY,
    CacheManagerCount: ULONG,
}}

STRUCT! {ETHREAD_3_1 {
    ForwardLinkShadow: P_VOID,
    StartAddress: P_VOID,
}}

STRUCT! {CLIENT_ID {
    UniqueProcess: HANDLE,
    UniqueThread: HANDLE,
}}

STRUCT! {PS_CLIENT_SECURITY_CONTEXT {
    Anonymous: PS_CLIENT_SECURITY_CONTEXT_1,
}}

STRUCT! {PSP_RATE_APC {
    Anonymous: PSP_RATE_APC_1,
    TargetGEneration: ULONGLONG,
    RateApc: KAPC,
}}

STRUCT! {EX_RUNDOWN_REF {
    Anonymous: EX_RUNDOWN_REF_1,
}}

STRUCT! {EX_PUSH_LOCK {
    Anonymous: EX_PUSH_LOCK_1,
}}

STRUCT! {KAPC {
    Type: UCHAR,
    SpareByte0: UCHAR,
    Size: UCHAR,
    SpareByte1: UCHAR,
    SpareLong0: ULONG,
    Thread: P_KTHREAD,
    ApcListEntry: LIST_ENTRY,
    KernelRoutine: P_VOID,
    RundownRoutine: P_VOID,
    NormalRoutine: P_VOID,
    NormalContext: P_VOID,
    SystemArgument1: P_VOID,
    SystemArgument2: P_VOID,
    ApcStateIndex: CHAR,
    ApcMode: CHAR,
    Inserted: UCHAR,
}}

STRUCT! {KTHREAD {
    Header: DISPATCHER_HEADER,
    CycleTime: ULONGLONG,
    HighCycleTime: ULONG,
    InitialStack: P_VOID,
    StackLimit: P_VOID,
    KernelStack: P_VOID,
    ThreadLock: ULONG,
    Anonymous: KTHREAD_1,
    Priority: CHAR,
    NextProcessor: WORD,
    DeferredProcessor: WORD,
    ApcQueueLock: ULONG,
    ContextSwitches: ULONG,
    State: UCHAR,
    NpxState: UCHAR,
    WaitIrql: UCHAR,
    WaitMode: CHAR,
    WaitStatus: LONG,
    Anonymous_1: KTHREAD_2,
    Anonymous_2: KTHREAD_3,
    WaitReason: UCHAR,
    SwapBusy: UCHAR,
    Alerted: [UCHAR; 2],
    Anonymous_3: KTHREAD_4,
    Queue: P_KQUEUE,
    WaitTime: ULONG,
    Anonymous_4: KTHREAD_5,
    Teb: P_VOID,
    Anonymous_5: KTHREAD_6,
    Anonymous_6: KTHREAD_7,
    Anonymous_7: KTHREAD_8,
    LargeStack: UCHAR,
    QueueListEntry: LIST_ENTRY,
    TrapFrame: P_KTRAP_FRAME,
    FirstArgument: P_VOID,
    Anonymous_8: KTHREAD_9,
    ServiceTable: P_VOID,
    ApcStateIndex: UCHAR,
    BasePriority: CHAR,
    PriorityDecrement: CHAR,
    Preempted: UCHAR,
    AdjustReason: UCHAR,
    AdjustIncrement: CHAR,
    Spare01: UCHAR,
    Saturation: CHAR,
    SystemCallNumber: ULONG,
    Spare02: UCHAR,
    UserAffinity: ULONG,
    Process: P_KPROCESS,
    Affinity: ULONG,
    ApcStatePointer: [P_KAPC_STATE; 2],
    Anonymous_9: KTHREAD_10,
    FreezeCount: CHAR,
    SuspendCount: CHAR,
    UserIdealProcessor: UCHAR,
    Spare03: UCHAR,
    Iopl: UCHAR,
    Win32Thread: P_VOID,
    StackBase: P_VOID,
    Anonymous_10: KTHREAD_11,
    PowerState: UCHAR,
    UserTime: ULONG,
    Anonymous_11: KTHREAD_12,
    SListFaultCount: ULONG,
    ThreadListEntry: LIST_ENTRY,
    MutantListHead: LIST_ENTRY,
    SListFaultAddress: P_VOID,
    MdlForLockedTeb: P_VOID,
}}

STRUCT! {KAPC_STATE {
    ApcListHead: [LIST_ENTRY; 2],
    Process: P_KPROCESS,
    KernelApcInProgress: UCHAR,
    KernelApcPending: UCHAR,
    UserApcPending: UCHAR,
}}

STRUCT! {KQUEUE {
    Header: DISPATCHER_HEADER,
    EntryListHead: LIST_ENTRY,
    CurrentCount: ULONG,
    MaximumCount: ULONG,
    ThreadListHead: LIST_ENTRY,
}}

STRUCT! {KTRAP_FRAME {
    DbgEbp: ULONG,
    DbgEip: ULONG,
    DbgArgMark: ULONG,
    DbgArgPointer: ULONG,
    TempSegCs: WORD,
    Logging: UCHAR,
    Reserved: UCHAR,
    TempEsp: ULONG,
    Dr0: ULONG,
    Dr1: ULONG,
    Dr2: ULONG,
    Dr3: ULONG,
    Dr4: ULONG,
    Dr5: ULONG,
    Dr6: ULONG,
    Dr7: ULONG,
    SegGs: ULONG,
    SegEs: ULONG,
    SegDs: ULONG,
    Edx: ULONG,
    Ecx: ULONG,
    Eax: ULONG,
    PreviousPreviousMode: ULONG,
    ExceptionList: P_EXCEPTION_REGISTRATION_RECORD,
    SegFs: ULONG,
    Edi: ULONG,
    Esi: ULONG,
    Ebx: ULONG,
    Ebp: ULONG,
    ErrCode: ULONG,
    Eip: ULONG,
    SegCs: ULONG,
    EFlags: ULONG,
    HardwareEsp: ULONG,
    HardwareSegSs: ULONG,
    V86Es: ULONG,
    V86Ds: ULONG,
    V86Fs: ULONG,
    V86Gs: ULONG,
}}

STRUCT! {EXCEPTION_REGISTRATION_RECORD {
    Next: P_EXCEPTION_REGISTRATION_RECORD,
    Handler: EXCEPTION_ROUTINE,
}}

STRUCT! {FILE_OBJECT {
    Type: SHORT,
    Size: SHORT,
    DeviceObject: P_DEVICE_OBJECT,
    Vpb: P_VPB,
    FsContext: P_VOID,
    FsContext2: P_VOID,
    SectionObjectPointer: P_SECTION_OBJECT_POINTERS,
    PrivateCacheMap: P_VOID,
    FinalStatus: NTSTATUS,
    RelatedFileObject: P_FILE_OBJECT,
    LockOperation: BOOLEAN,
    DeletePending: BOOLEAN,
    ReadAccess: BOOLEAN,
    WriteAccess: BOOLEAN,
    DeleteAccess: BOOLEAN,
    SharedRead: BOOLEAN,
    SharedWrite: BOOLEAN,
    SharedDelete: BOOLEAN,
    Flags: ULONG,
    FileName: UNICODE_STRING,
    CurrentByteOffset: LARGE_INTEGER,
    Waiters: ULONG,
    Busy: ULONG,
    LastLock: P_VOID,
    Event: KEVENT,
    CompletionContext: P_IO_COMPLETION_CONTEXT,
    IrpListLock: KSPIN_LOCK,
    IrpList: LIST_ENTRY,
    FileObjectExtension: P_IOP_FILE_OBJECT_EXTENSION,
}}

STRUCT! {SECTION_OBJECT_POINTERS {
    DataSectionObject: P_VOID,
    SharedCacheMap: P_VOID,
    ImageSectionObject: P_VOID,
}}

STRUCT! {IO_COMPLETION_CONTEXT {
    Port: P_VOID,
    Key: P_VOID,
}}

STRUCT! {IOP_FILE_OBJECT_EXTENSION {
    FoExtFlags: ULONG,
    FoExtPerTypeExtension: [P_VOID; 7],
    FoIoPriorityHint: IOP_PRIORITY_HINT,
}}

STRUCT! {SID_IDENTIFIER_AUTHORITY {
    Value: [BYTE; 6],
}}

STRUCT! {DISPATCHER_HEADER {
    Anonymous: DISPATCHER_HEADER_1,
    SignalState: LONG,
    WaitListHead: LIST_ENTRY,
}}

STRUCT! {DISPATCHER_HEADER_1_1 {
    Type: UCHAR,
    Anonymous: DISPATCHER_HEADER_1_1_1,
    Anonymous_1: DISPATCHER_HEADER_1_1_2,
    Anonymous_2: DISPATCHER_HEADER_1_1_3,
}}

STRUCT! {EPROCESS {
    Pcb: KPROCESS,
    ProcessLock: EX_PUSH_LOCK,
    CreateTime: LARGE_INTEGER,
    ExitTime: LARGE_INTEGER,
    RundownProtect: EX_RUNDOWN_REF,
    UniqueProcessId: P_VOID,
    ActiveProcessLinks: LIST_ENTRY,
    QuotaUsage: [ULONG; 3],
    QuotaPeak: [ULONG; 3],
    CommitCharge: ULONG,
    PeakVirtualSize: ULONG,
    VirtualSize: ULONG,
    SessionProcessLinks: LIST_ENTRY,
    DebugPort: P_VOID,
    Anonymous: EPROCESS_1,
    ObjectTable: P_HANDLE,
    Token: EX_FAST_REF,
    WorkingSetPage: ULONG,
    AddressCreationLock: EX_PUSH_LOCK,
    RotateInProgress: P_ETHREAD,
    ForkInProgress: P_ETHREAD,
    HardwareTrigger: ULONG,
    PhysicalVadRoot: P_MM_AVL_TABLE,
    CloneRoot: P_VOID,
    NumberOfPrivatePages: ULONG,
    NumberOfLockedPages: ULONG,
    Win32Process: P_VOID,
    Job: P_EJOB,
    SectionObject: P_VOID,
    SectionBaseAddress: P_VOID,
    QuotaBlock: P_EPROCESS_QUOTA_BLOCK,
    WorkingSetWatch: P_PAGEFAULT_HISTORY,
    Win32WindowStation: P_VOID,
    InheritedFromUniqueProcessId: P_VOID,
    LdtInformation: P_VOID,
    VadFreeHint: P_VOID,
    VdmObjects: P_VOID,
    DeviceMap: P_VOID,
    EtwDataSource: P_VOID,
    FreeTebHint: P_VOID,
    Anonymous_1: EPROCESS_2,
    Session: P_VOID,
    ImageFileName: [UCHAR; 16],
    JobLinks: LIST_ENTRY,
    LockedPagesList: P_VOID,
    ThreadListHead: LIST_ENTRY,
    SecurityPort: P_VOID,
    PaeTop: P_VOID,
    ActiveThreads: ULONG,
    ImagePathHash: ULONG,
    DefaultHardErrorProcessing: ULONG,
    LastThreadExitStatus: LONG,
    Peb: P_PEB,
    PrefetchTrace: EX_FAST_REF,
    ReadOperationCount: LARGE_INTEGER,
    WriteOperationCount: LARGE_INTEGER,
    OtherOperationCount: LARGE_INTEGER,
    ReadTransferCount: LARGE_INTEGER,
    WriteTransferCount: LARGE_INTEGER,
    OtherTransferCount: LARGE_INTEGER,
    CommitChargeLimit: ULONG,
    CommitChargePeak: ULONG,
    AweInfo: P_VOID,
    SeAuditProcessCreationInfo: SE_AUDIT_PROCESS_CREATION_INFO,
    Vm: MMSUPPORT,
    MmProcessLinks: LIST_ENTRY,
    ModifiedPageCount: ULONG,
    Flags2: ULONG,
    Flags_0: ULONG,
    Flags: ULONG,
    Flags_1: ULONG,
    ExitStatus: LONG,
    Spare7: WORD,
    Anonymous_2: EPROCESS_3,
    PriorityClass: UCHAR,
    VadRoot: MM_AVL_TABLE,
    Cookie: ULONG,
    AlpcContext: ALPC_PROCESS_CONTEXT,
}}

STRUCT! {EPROCESS_3_1 {
    SubSystemMinorVersion: UCHAR,
    SubSystemMajorVersion: UCHAR,
}}

STRUCT! {EX_FAST_REF {
    Anonymous: EX_FAST_REF_1,
}}

STRUCT! {MM_AVL_TABLE {
    BalancedRoot: MMADDRESS_NODE,
    Flags_0: ULONG,
    NodeHint: P_VOID,
    NodeFreeHint: P_VOID,
}}

STRUCT! {EPROCESS_QUOTA_BLOCK {
    QuotaEntry: [EPROCESS_QUOTA_ENTRY; PsQuotaTypes as usize],
    QuotaList: LIST_ENTRY,
    ReferenceCount: ULONG,
    ProcessCount: ULONG,
}}

STRUCT! {EPROCESS_QUOTA_ENTRY {
    Usage: SIZE_T,
    Limit: SIZE_T,
    Peak: SIZE_T,
    Return: SIZE_T,
}}

STRUCT! {PAGEFAULT_HISTORY {
    CurrentIndex: ULONG,
    MapIndex: ULONG,
    SpinLock: KSPIN_LOCK,
    Reserved: P_VOID,
    WatchInfo: [PROCESS_WS_WATCH_INFORMATION; 1],
}}

STRUCT! {SE_AUDIT_PROCESS_CREATION_INFO {
    ImageFileName: P_OBJECT_NAME_INFORMATION,
}}

STRUCT! {KPROCESS {
    Header: DISPATCHER_HEADER,
    ProfileListHead: LIST_ENTRY,
    DirectoryTableBase: ULONG,
    Unused0: ULONG,
    LdtDescriptor: KGDTENTRY,
    Int21Descriptor: KIDTENTRY,
    IopmOffset: WORD,
    Iopl: UCHAR,
    Unused: UCHAR,
    ActiveProcessors: ULONG,
    KernelTime: ULONG,
    UserTime: ULONG,
    ReadyListHead: LIST_ENTRY,
    SwapListEntry: SINGLE_LIST_ENTRY,
    VdmTrapcHandler: P_VOID,
    ThreadListHead: LIST_ENTRY,
    ProcessLock: ULONG,
    Affinity: ULONG,
    Anonymous: KPROCESS_1,
    BasePriority: CHAR,
    QuantumReset: CHAR,
    State: UCHAR,
    ThreadSeed: UCHAR,
    PowerState: UCHAR,
    IdealNode: UCHAR,
    Visited: UCHAR,
    Anonymous_1: KPROCESS_2,
    StackCount: ULONG,
    ProcessListEntry: LIST_ENTRY,
    CycleTime: ULONG,
}}

STRUCT! {EJOB {
    Event: KEVENT,
    JobLinks: LIST_ENTRY,
    ProcessListHead: LIST_ENTRY,
    JobLock: ERESOURCE,
    TotalUserTime: LARGE_INTEGER,
    TotalKernelTime: LARGE_INTEGER,
    ThisPeriodTotalUserTime: LARGE_INTEGER,
    ThisPeriodTotalKernelTime: LARGE_INTEGER,
    TotalPageFaultCount: ULONG,
    TotalProcesses: ULONG,
    ActiveProcesses: ULONG,
    TotalTerminatedProcesses: ULONG,
    PerProcessUserTimeLimit: LARGE_INTEGER,
    PerJobUserTimeLimit: LARGE_INTEGER,
    LimitFlags: ULONG,
    MinimumWorkingSetSize: ULONG,
    MaximumWorkingSetSize: ULONG,
    ActiveProcessLimit: ULONG,
    Affinity: ULONG,
    PriorityClass: UCHAR,
    AccessState: P_JOB_ACCESS_STATE,
    UIRestrictionsClass: ULONG,
    EndOfJobTimeAction: ULONG,
    CompletionPort: P_VOID,
    CompletionKey: P_VOID,
    SessionId: ULONG,
    SchedulingClass: ULONG,
    ReadOperationCount: ULONG,
    WriteOperationCount: ULONG,
    OtherOperationCount: ULONG,
    ReadTransferCount: ULONG,
    WriteTransferCount: ULONG,
    OtherTransferCount: ULONG,
    ProcessMemoryLimit: ULONG,
    JobMemoryLimit: ULONG,
    PeakProcessMemoryUsed: ULONG,
    CurrentJobMemoryUsed: ULONG,
    MemoryLimitsLock: EX_PUSH_LOCK,
    JobSetLinks: LIST_ENTRY,
    MemberLevel: ULONG,
    JobFlags: ULONG,
}}

STRUCT! {PEB {
    Reserved1: [BYTE; 2],
    BeingDebugged: BYTE,
    Reserved2: [BYTE; 1],
    Reserved3: [P_VOID; 2],
    Ldr: P_PEB_LDR_DATA,
    ProcessParameters: P_RTL_USER_PROCESS_PARAMETERS,
    Reserved4: [P_VOID; 3],
    AtlThunkSListPtr: P_VOID,
    Reserved5: P_VOID,
    Reserved6: ULONG,
    Reserved7: P_VOID,
    Reserved8: ULONG,
    AtlThunkSListPtr32: ULONG,
    Reserved9: [P_VOID; 45],
    Reserved10: [BYTE; 96],
    PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    Reserved11: [BYTE; 128],
    Reserved12: [P_VOID; 1],
    SessionId: ULONG,
}}

STRUCT! {PEB_LDR_DATA {
    Reserved1: [BYTE; 8],
    Reserved2: [P_VOID; 3],
    InMemoryOrderModuleList: LIST_ENTRY,
}}

STRUCT! {RTL_USER_PROCESS_PARAMETERS {
    Reserved1: [BYTE; 16],
    Reserved2: [P_VOID; 10],
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
}}

STRUCT! {MMSUPPORT {
    WorkingSetExpansionLinks: LIST_ENTRY,
    LastTrimStamp: WORD,
    NextPageColor: WORD,
    Flags: MMSUPPORT_FLAGS,
    PageFaultCount: ULONG,
    PeakWorkingSetSize: ULONG,
    Spare0: ULONG,
    MinimumWorkingSetSize: ULONG,
    MaximumWorkingSetSize: ULONG,
    VmWorkingSetList: P_MMWSL,
    Claim: ULONG,
    Spare: [ULONG; 1],
    WorkingSetPrivateSize: ULONG,
    WorkingSetSizeOverhead: ULONG,
    WorkingSetSize: ULONG,
    ExitEvent: P_KEVENT,
    WorkingSetMutex: EX_PUSH_LOCK,
    AccessLog: P_VOID,
}}

STRUCT! {ALPC_PROCESS_CONTEXT {
    Lock: EX_PUSH_LOCK,
    ViewListHead: LIST_ENTRY,
    PagedPoolQuotaCache: ULONG,
}}

STRUCT! {MMADDRESS_NODE {
    u1: ULONG,
    LeftChild: P_MMADDRESS_NODE,
    RightChild: P_MMADDRESS_NODE,
    StartingVpn: ULONG,
    EndingVpn: ULONG,
}}

STRUCT! {PROCESS_WS_WATCH_INFORMATION {
    FaultingPc: P_VOID,
    FaultingVa: P_VOID,
}}

STRUCT! {OBJECT_NAME_INFORMATION {
    Name: UNICODE_STRING,
}}

STRUCT! {KGDTENTRY {
    LimitLow: WORD,
    BaseLow: WORD,
    HighWord: ULONG,
}}

STRUCT! {KIDTENTRY {
    Offset: WORD,
    Selector: WORD,
    Access: WORD,
    ExtendedOffset: WORD,
}}

STRUCT! {ERESOURCE {
    SystemResourcesList: LIST_ENTRY,
    OwnerTable: P_OWNER_ENTRY,
    ActiveCount: SHORT,
    Flag: WORD,
    SharedWaiters: P_KSEMAPHORE,
    ExclusiveWaiters: P_KEVENT,
    OwnerEntry: OWNER_ENTRY,
    ActiveEntries: ULONG,
    ContentionCount: ULONG,
    NumberOfSharedWaiters: ULONG,
    NumberOfExclusiveWaiters: ULONG,
    Anonymous: ERESOURCE_1,
    SpinLock: ULONG,
}}

STRUCT! {JOB_ACCESS_STATE {}}

STRUCT! {MMSUPPORT_FLAGS {
    Flags_0: ULONG,
}}

STRUCT! {MMWSL {
    FirstFree: ULONG,
    FirstDynamic: ULONG,
    LastEntry: ULONG,
    NextSlot: ULONG,
    Wsle: P_MMWSLE,
    LowestPagableAddress: P_VOID,
    LastInitializedWsle: ULONG,
    NextEstimationSlot: ULONG,
    NextAgingSlot: ULONG,
    EstimatedAvailable: ULONG,
    GrowthSinceLastEstimate: ULONG,
    NumberOfCommittedPageTables: ULONG,
    VadBitMapHint: ULONG,
    NonDirectCount: ULONG,
    NonDirectHash: P_MMWSLE_NONDIRECT_HASH,
    HashTableStart: P_MMWSLE_HASH,
    HighestPermittedHashAddress: P_MMWSLE_HASH,
    HighestUserAddress: P_VOID,
    UsedPageTableEntries: [WORD; 1536],
    CommittedPageTables: [ULONG; 48],
}}

STRUCT! {OWNER_ENTRY {
    OwnerThread: ULONG,
    Anonymous: OWNER_ENTRY_1,
}}

STRUCT! {KSEMAPHORE {
    Header: DISPATCHER_HEADER,
    Limit: LONG,
}}

STRUCT! {MMWSLE {
    u1: ULONG,
}}

STRUCT! {MMWSLE_NONDIRECT_HASH {
    Key: P_VOID,
    Index: ULONG,
}}

STRUCT! {MMWSLE_HASH {
    Index: ULONG,
}}

STRUCT! {WAIT_CONTEXT_BLOCK {
    Anonymous: WAIT_CONTEXT_BLOCK_1,
    DeviceRoutine: P_DRIVER_CONTROL,
    DeviceContext: P_VOID,
    NumberOfMapRegisters: ULONG,
    DeviceObject: P_VOID,
    CurrentIrp: P_VOID,
    BufferChainingDpc: P_KDPC,
}}

STRUCT! {WAIT_CONTEXT_BLOCK_1_1 {
    DmaWaitEntry: LIST_ENTRY,
    NumberOfChannels: ULONG,
    Flags_0: ULONG,
}}

STRUCT! {KDEVICE_QUEUE_ENTRY {
    DeviceListEntry: LIST_ENTRY,
    SortKey: ULONG,
    Inserted: UCHAR,
}}

STRUCT! {IO_STACK_LOCATION {
    MajorFunction: UCHAR,
    MinorFunction: UCHAR,
    Flags: UCHAR,
    Control: UCHAR,
    Parameters: IO_STACK_LOCATION_1,
    DeviceObject: P_DEVICE_OBJECT,
    FileObject: P_FILE_OBJECT,
    CompletionRoutine: P_IO_COMPLETION_ROUTINE,
    Context: P_VOID,
}}

STRUCT! {IORING_OBJECT {
    Type: USHORT,
    Size: USHORT,
    Info: NT_IORING_INFO,
    SectionObject: P_SECTION,
    KernelMappedBase: P_VOID,
    Mdl: P_MDL,
    MdlMappedBase: P_VOID,
    ViewSize: ULONG,
    SubmitInProgress: ULONG,
    IoRingEntryLock: P_VOID,
    EntriesCompleted: P_VOID,
    EntriesSubmitted: P_VOID,
    RingEvent: KEVENT,
    EntriesPending: P_VOID,
    BuffersRegistered: ULONG,
    BufferArray: P_IORING_BUFFER_INFO,
    FilesRegistered: ULONG,
    FileHandleArray: P_HANDLE,
}}

STRUCT! {TERMINATION_PORT {
    Next: P_TERMINATION_PORT,
    Port: P_VOID,
}}

STRUCT! {KWAIT_BLOCK {
    WaitListEntry: LIST_ENTRY,
    Thread: P_KTHREAD,
    Object: P_VOID,
    NextWaitBlock: P_KWAIT_BLOCK,
    WaitKey: WORD,
    WaitType: UCHAR,
    SpareByte: UCHAR,
}}

STRUCT! {KGATE {
    Header: DISPATCHER_HEADER,
}}

STRUCT! {KTHREAD_5_1 {
    KernelApcDisable: SHORT,
    SpecialApcDisable: SHORT,
}}

STRUCT! {KTIMER {
    Header: DISPATCHER_HEADER,
    DueTime: ULARGE_INTEGER,
    TimerListEntry: LIST_ENTRY,
    Dpc: P_KDPC,
    Period: LONG,
}}
