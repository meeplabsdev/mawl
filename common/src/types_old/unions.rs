#![allow(nonstandard_style)]
use crate::{
    UNION,
    types_old::{
        structs::{
            DISPATCHER_HEADER_1_1, EPROCESS_3_1, ETHREAD_3_1, KAPC, KAPC_STATE,
            KDEVICE_QUEUE_ENTRY, KSEMAPHORE, KTHREAD_5_1, LARGE_INTEGER_PART, LIST_ENTRY,
            OVERLAY_IRP_1, P_ETHREAD, P_IO_STACK_LOCATION, P_IO_STATUS_BLOCK, P_IORING_OBJECT,
            P_IRP, P_KGATE, P_KWAIT_BLOCK, P_TERMINATION_PORT, SINGLE_LIST_ENTRY, TAIL_OVERLAY_IRP,
            TAIL_OVERLAY_IRP_3, ULARGE_INTEGER_PART, WAIT_CONTEXT_BLOCK, WAIT_CONTEXT_BLOCK_1_1,
        },
        types::{
            LONG, LONGLONG, NTSTATUS, P_IO_APC_ROUTINE, P_VOID, UCHAR, ULONG, ULONGLONG, WORD,
        },
    },
};

UNION! {LARGE_INTEGER {
    s: LARGE_INTEGER_PART,
    u: LARGE_INTEGER_PART,
    QuadPart: LONGLONG,
}}

UNION! {ULARGE_INTEGER {
    s: ULARGE_INTEGER_PART,
    u: ULARGE_INTEGER_PART,
    QuadPart: ULONGLONG,
}}

UNION! {QUEUE {
    ListEntry: LIST_ENTRY,
    Wcb: WAIT_CONTEXT_BLOCK,
}}

UNION! {ASSOCIATED_IRP {
    MasterIrp: P_IRP,
    IrpCount: LONG,
    SystemBuffer: P_VOID,
}}

UNION! {IRP_1 {
    UserIosb: P_IO_STATUS_BLOCK,
    IoRingContext: P_VOID,
}}

UNION! {OVERLAY_IRP {
    AsynchronousParameters: OVERLAY_IRP_1,
    AllocationSize: LARGE_INTEGER,
}}

UNION! {TAIL_IRP {
    Overlay: TAIL_OVERLAY_IRP,
    Apc: KAPC,
    CompletionKey: P_VOID,
}}

UNION! {TAIL_OVERLAY_IRP_1 {
    DeviceQueueEntry: KDEVICE_QUEUE_ENTRY,
    Anonymous: TAIL_OVERLAY_IRP_3,
}}

UNION! {TAIL_OVERLAY_IRP_4 {
    CurrentStackLocation: P_IO_STACK_LOCATION,
    PacketType: ULONG,
}}

UNION! {OVERLAY_IRP_2 {
    UserApcRoutine: P_IO_APC_ROUTINE,
    IssuingProcess: P_VOID,
}}

UNION! {OVERLAY_IRP_3 {
    UserApcContext: P_VOID,
    IoRing: P_IORING_OBJECT,
}}

UNION! {IO_STATUS_BLOCK_1 {
    Status: NTSTATUS,
    Pointer: P_VOID,
}}

UNION! {ETHREAD_1 {
    ExitTime: LARGE_INTEGER,
    KeyedWaitChain: LIST_ENTRY,
}}

UNION! {ETHREAD_2 {
    ExitStatus: LONG,
    OfsChain: P_VOID,
}}

UNION! {ETHREAD_3 {
    PostBlockList: LIST_ENTRY,
    Anonymous: ETHREAD_3_1,
}}

UNION! {ETHREAD_4 {
    TerminationPort: P_TERMINATION_PORT,
    ReaperLink: P_ETHREAD,
    KeyedWaitValue: P_VOID,
    Win32StartParameter: P_VOID,
}}

UNION! {ETHREAD_5 {
    KeyedWaitSemaphore: KSEMAPHORE,
    AlpcWaitSemaphore: KSEMAPHORE,
}}

UNION! {ETHREAD_6 {
    AlpcMessage: P_VOID,
    AlpcReceiveAttributeSet: ULONG,
}}

UNION! {PS_CLIENT_SECURITY_CONTEXT_1 {
    ImpersonationData: ULONG,
    ImpersonationToken: P_VOID,
    Flags_0: ULONG,
}}

UNION! {PSP_RATE_APC_1 {
    NextApc: SINGLE_LIST_ENTRY,
    ExcessCycles: ULONGLONG,
}}

UNION! {EX_RUNDOWN_REF_1 {
    Count: ULONG,
    Ptr: P_VOID,
}}

UNION! {EX_PUSH_LOCK_1 {
    Flags_0: ULONG,
    Value: ULONG,
    Ptr: P_VOID,
}}

UNION! {KTHREAD_1 {
    ApcState: KAPC_STATE,
    ApcStateFill: [UCHAR; 23],
}}

UNION! {KTHREAD_2 {
    WaitBlockList: P_KWAIT_BLOCK,
    GateObject: P_KGATE,
}}

UNION! {KTHREAD_3 {
    Flags_0: ULONG,
    MiscFlags: LONG,
}}

UNION! {KTHREAD_4 {
    WaitListEntry: LIST_ENTRY,
    SwapListEntry: SINGLE_LIST_ENTRY,
}}

UNION! {KTHREAD_5 {
    Anonymous: KTHREAD_5_1,
    CombinedApcDisable: ULONG,
}}

UNION! {KTHREAD_6 {
    Timer: KTIMER,
    TimerFill: [UCHAR; 40],
}}

UNION! {KTHREAD_7 {
    Flags_0: ULONG,
    ThreadFlags: LONG,
}}

UNION! {KTHREAD_8 {
    WaitBlock: [KWAIT_BLOCK; 4],
    Anonymous: KTHREAD_8_1,
    Anonymous_1: KTHREAD_8_2,
    Anonymous_2: KTHREAD_8_3,
    WaitBlockFill3: [UCHAR; 95],
}}

//           struct
//           {
//                UCHAR WaitBlockFill0[23];
//                UCHAR IdealProcessor;
//           } KTHREAD_8_1;
//           struct
//           {
//                UCHAR WaitBlockFill1[47];
//                CHAR PreviousMode;
//           } KTHREAD_8_2;
//           struct
//           {
//                UCHAR WaitBlockFill2[71];
//                UCHAR ResourceIndex;
//           } KTHREAD_8_3;

UNION! {KTHREAD_9 {
    CallbackStack: P_VOID,
    CallbackDepth: ULONG,
}}

UNION! {KTHREAD_10 {
    SavedApcState: KAPC_STATE,
    SavedApcStateFill: [UCHAR; 23],
}}

UNION! {KTHREAD_11 {
    SuspendApc: KAPC,
    Anonymous: KTHREAD_11_1,
    Anonymous_1: KTHREAD_11_2,
    Anonymous_2: KTHREAD_11_3,
    Anonymous_3: KTHREAD_11_4,
    Anonymous_4: KTHREAD_11_5,
    SuspendApcFill5: [UCHAR; 47],
}}

//           struct
//           {
//                UCHAR SuspendApcFill0[1];
//                CHAR Spare04;
//           } KTHREAD_11_1;
//           struct
//           {
//                UCHAR SuspendApcFill1[3];
//                UCHAR QuantumReset;
//           } KTHREAD_11_2;
//           struct
//           {
//                UCHAR SuspendApcFill2[4];
//                ULONG KernelTime;
//           } KTHREAD_11_3;
//           struct
//           {
//                UCHAR SuspendApcFill3[36];
//                PKPRCB WaitPrcb;
//           } KTHREAD_11_4;
//           struct
//           {
//                UCHAR SuspendApcFill4[40];
//                PVOID LegoData;
//           } KTHREAD_11_5;

UNION! {KTHREAD_12 {
    SuspendSemaphore: KSEMAPHORE,
    SuspendSemaphorefill: [UCHAR; 20],
}}

UNION! {KPROCESS_1 {
    Flags_0: ULONG,
    ProcessFlags: LONG,
}}

UNION! {KPROCESS_2 {
    Flags: KEXECUTE_OPTIONS,
    ExecuteOptions: UCHAR,
}}

UNION! {DISPATCHER_HEADER_1 {
    Anonymous: DISPATCHER_HEADER_1_1,
    Lock: LONG,
}}

UNION! {DISPATCHER_HEADER_1_1_1 {
    Abandoned: UCHAR,
    Absolute: UCHAR,
    NpxIrql: UCHAR,
    Signalling: UCHAR,
}}

UNION! {DISPATCHER_HEADER_1_1_2 {
    Size: UCHAR,
    Hand: UCHAR,
}}

UNION! {DISPATCHER_HEADER_1_1_3 {
    Inserted: UCHAR,
    DebugActive: UCHAR,
    DpcActive: UCHAR,
}}

UNION! {EPROCESS_1 {
    ExceptionPortData: P_VOID,
    ExceptionPortValue: ULONG,
    Flags_0: ULONG,
}}

UNION! {EPROCESS_2 {
    PageDirectoryPte: HARDWARE_PTE,
    Filler: ULONGLONG,
}}

UNION! {EPROCESS_3 {
    Anonymous: EPROCESS_3_1,
    SubSystemVersion: WORD,
}}

UNION! {EX_FAST_REF_1 {
    Object: P_VOID,
    Flags_0: ULONG,
    Value: ULONG,
}}

UNION! {ERESOURCE_1 {
    Address: P_VOID,
    CreatorBackTraceIndex: ULONG,
}}

UNION! {OWNER_ENTRY_1 {
    OwnerCount: LONG,
    TableSize: ULONG,
}}

UNION! {WAIT_CONTEXT_BLOCK_1 {
    WaitQueueEntry: KDEVICE_QUEUE_ENTRY,
    Anonymous: WAIT_CONTEXT_BLOCK_1_1
}}

UNION! {IO_STACK_LOCATION_1 {
    Create: IO_STACK_LOCATION_1_1,
    CreatePipe: IO_STACK_LOCATION_1_2,
    CreateMailslot: IO_STACK_LOCATION_1_3,
    Read: IO_STACK_LOCATION_1_4,
    Write: IO_STACK_LOCATION_1_5,
    QueryDirectory: IO_STACK_LOCATION_1_6,
    NotifyDirectory: IO_STACK_LOCATION_1_7,
    NotifyDirectoryEx: IO_STACK_LOCATION_1_8,
    QueryFile: IO_STACK_LOCATION_1_9,
    SetFile: IO_STACK_LOCATION_1_10,
    QueryEa: IO_STACK_LOCATION_1_11,
    SetEa: IO_STACK_LOCATION_1_12,
    QueryVolume: IO_STACK_LOCATION_1_13,
    SetVolume: IO_STACK_LOCATION_1_14,
    FileSystemControl: IO_STACK_LOCATION_1_15,
    LockControl: IO_STACK_LOCATION_1_16,
    DeviceIoControl: IO_STACK_LOCATION_1_17,
    QuerySecurity: IO_STACK_LOCATION_1_18,
    SetSecurity: IO_STACK_LOCATION_1_19,
    MountVolume: IO_STACK_LOCATION_1_20,
    VerifyVolume: IO_STACK_LOCATION_1_21,
    Scsi: IO_STACK_LOCATION_1_22,
    QueryQuota: IO_STACK_LOCATION_1_23,
    SetQuota: IO_STACK_LOCATION_1_24,
    QueryDeviceRelations: IO_STACK_LOCATION_1_25,
    QueryInterface: IO_STACK_LOCATION_1_26,
    DeviceCapabilities: IO_STACK_LOCATION_1_27,
    FilterResourceRequirements: IO_STACK_LOCATION_1_28,
    ReadWriteConfig: IO_STACK_LOCATION_1_29,
    SetLock: IO_STACK_LOCATION_1_30,
    QueryId: IO_STACK_LOCATION_1_31,
    QueryDeviceText: IO_STACK_LOCATION_1_32,
    UsageNotification: IO_STACK_LOCATION_1_33,
    WaitWake: IO_STACK_LOCATION_1_34,
    PowerSequence: IO_STACK_LOCATION_1_35,
    Power: IO_STACK_LOCATION_1_36,
    StartDevice: IO_STACK_LOCATION_1_37,
    WMI: IO_STACK_LOCATION_1_38,
    Others: IO_STACK_LOCATION_1_39,
}}

// struct {
//     PIO_SECURITY_CONTEXT     SecurityContext;
//     ULONG                    Options;
//     USHORT POINTER_ALIGNMENT FileAttributes;
//     USHORT                   ShareAccess;
//     ULONG POINTER_ALIGNMENT  EaLength;
// } IO_STACK_LOCATION_1_1;
// struct {
//     PIO_SECURITY_CONTEXT          SecurityContext;
//     ULONG                         Options;
//     USHORT POINTER_ALIGNMENT      Reserved;
//     USHORT                        ShareAccess;
//     PNAMED_PIPE_CREATE_PARAMETERS Parameters;
// } IO_STACK_LOCATION_1_2;
// struct {
//     PIO_SECURITY_CONTEXT        SecurityContext;
//     ULONG                       Options;
//     USHORT POINTER_ALIGNMENT    Reserved;
//     USHORT                      ShareAccess;
//     PMAILSLOT_CREATE_PARAMETERS Parameters;
// } IO_STACK_LOCATION_1_3;
// struct {
//     ULONG                   Length;
//     ULONG POINTER_ALIGNMENT Key;
//     ULONG                   Flags;
//     LARGE_INTEGER           ByteOffset;
// } IO_STACK_LOCATION_1_4;
// struct {
//     ULONG                   Length;
//     ULONG POINTER_ALIGNMENT Key;
//     ULONG                   Flags;
//     LARGE_INTEGER           ByteOffset;
// } IO_STACK_LOCATION_1_5;
// struct {
//     ULONG                   Length;
//     PUNICODE_STRING         FileName;
//     FILE_INFORMATION_CLASS  FileInformationClass;
//     ULONG POINTER_ALIGNMENT FileIndex;
// } IO_STACK_LOCATION_1_6;
// struct {
//     ULONG                   Length;
//     ULONG POINTER_ALIGNMENT CompletionFilter;
// } IO_STACK_LOCATION_1_7;
// struct {
//     ULONG                                                Length;
//     ULONG POINTER_ALIGNMENT                              CompletionFilter;
//     DIRECTORY_NOTIFY_INFORMATION_CLASS POINTER_ALIGNMENT DirectoryNotifyInformationClass;
// } IO_STACK_LOCATION_1_8;
// struct {
//     ULONG                                    Length;
//     FILE_INFORMATION_CLASS POINTER_ALIGNMENT FileInformationClass;
// } IO_STACK_LOCATION_1_9;
// struct {
//     ULONG                                    Length;
//     FILE_INFORMATION_CLASS POINTER_ALIGNMENT FileInformationClass;
//     PFILE_OBJECT                             FileObject;
//     union {
//     struct {
//         BOOLEAN ReplaceIfExists;
//         BOOLEAN AdvanceOnly;
//     };
//     ULONG  ClusterCount;
//     HANDLE DeleteHandle;
//     };
// } IO_STACK_LOCATION_1_10;
// struct {
//     ULONG                   Length;
//     PVOID                   EaList;
//     ULONG                   EaListLength;
//     ULONG POINTER_ALIGNMENT EaIndex;
// } IO_STACK_LOCATION_1_11;
// struct {
//     ULONG Length;
// } IO_STACK_LOCATION_1_12;
// struct {
//     ULONG                                  Length;
//     FS_INFORMATION_CLASS POINTER_ALIGNMENT FsInformationClass;
// } IO_STACK_LOCATION_1_13;
// struct {
//     ULONG                                  Length;
//     FS_INFORMATION_CLASS POINTER_ALIGNMENT FsInformationClass;
// } IO_STACK_LOCATION_1_14;
// struct {
//     ULONG                   OutputBufferLength;
//     ULONG POINTER_ALIGNMENT InputBufferLength;
//     ULONG POINTER_ALIGNMENT FsControlCode;
//     PVOID                   Type3InputBuffer;
// } IO_STACK_LOCATION_1_15;
// struct {
//     PLARGE_INTEGER          Length;
//     ULONG POINTER_ALIGNMENT Key;
//     LARGE_INTEGER           ByteOffset;
// } IO_STACK_LOCATION_1_16;
// struct {
//     ULONG                   OutputBufferLength;
//     ULONG POINTER_ALIGNMENT InputBufferLength;
//     ULONG POINTER_ALIGNMENT IoControlCode;
//     PVOID                   Type3InputBuffer;
// } IO_STACK_LOCATION_1_17;
// struct {
//     SECURITY_INFORMATION    SecurityInformation;
//     ULONG POINTER_ALIGNMENT Length;
// } IO_STACK_LOCATION_1_18;
// struct {
//     SECURITY_INFORMATION SecurityInformation;
//     PSECURITY_DESCRIPTOR SecurityDescriptor;
// } IO_STACK_LOCATION_1_19;
// struct {
//     PVPB           Vpb;
//     PDEVICE_OBJECT DeviceObject;
//     ULONG          OutputBufferLength;
// } IO_STACK_LOCATION_1_20;
// struct {
//     PVPB           Vpb;
//     PDEVICE_OBJECT DeviceObject;
// } IO_STACK_LOCATION_1_21;
// struct {
//     struct _SCSI_REQUEST_BLOCK *Srb;
// } IO_STACK_LOCATION_1_22;
// struct {
//     ULONG                       Length;
//     PSID                        StartSid;
//     PFILE_GET_QUOTA_INFORMATION SidList;
//     ULONG                       SidListLength;
// } IO_STACK_LOCATION_1_23;
// struct {
//     ULONG Length;
// } IO_STACK_LOCATION_1_24;
// struct {
//     DEVICE_RELATION_TYPE Type;
// } IO_STACK_LOCATION_1_25;
// struct {
//     const GUID *InterfaceType;
//     USHORT     Size;
//     USHORT     Version;
//     PINTERFACE Interface;
//     PVOID      InterfaceSpecificData;
// } IO_STACK_LOCATION_1_26;
// struct {
//     PDEVICE_CAPABILITIES Capabilities;
// } IO_STACK_LOCATION_1_27;
// struct {
//     PIO_RESOURCE_REQUIREMENTS_LIST IoResourceRequirementList;
// } IO_STACK_LOCATION_1_28;
// struct {
//     ULONG                   WhichSpace;
//     PVOID                   Buffer;
//     ULONG                   Offset;
//     ULONG POINTER_ALIGNMENT Length;
// } IO_STACK_LOCATION_1_29;
// struct {
//     BOOLEAN Lock;
// } IO_STACK_LOCATION_1_30;
// struct {
//     BUS_QUERY_ID_TYPE IdType;
// } IO_STACK_LOCATION_1_31;
// struct {
//     DEVICE_TEXT_TYPE       DeviceTextType;
//     LCID POINTER_ALIGNMENT LocaleId;
// } IO_STACK_LOCATION_1_32;
// struct {
//     BOOLEAN                                          InPath;
//     BOOLEAN                                          Reserved[3];
//     DEVICE_USAGE_NOTIFICATION_TYPE POINTER_ALIGNMENT Type;
// } IO_STACK_LOCATION_1_33;
// struct {
//     SYSTEM_POWER_STATE PowerState;
// } IO_STACK_LOCATION_1_34;
// struct {
//     PPOWER_SEQUENCE PowerSequence;
// } IO_STACK_LOCATION_1_35;
// struct {
//     union {
//     ULONG                      SystemContext;
//     SYSTEM_POWER_STATE_CONTEXT SystemPowerStateContext;
//     };
//     POWER_STATE_TYPE POINTER_ALIGNMENT Type;
//     POWER_STATE POINTER_ALIGNMENT      State;
//     POWER_ACTION POINTER_ALIGNMENT     ShutdownType;
// } IO_STACK_LOCATION_1_36;
// struct {
//     PCM_RESOURCE_LIST AllocatedResources;
//     PCM_RESOURCE_LIST AllocatedResourcesTranslated;
// } IO_STACK_LOCATION_1_37;
// struct {
//     ULONG_PTR ProviderId;
//     PVOID     DataPath;
//     ULONG     BufferSize;
//     PVOID     Buffer;
// } IO_STACK_LOCATION_1_38;
// struct {
//     PVOID Argument1;
//     PVOID Argument2;
//     PVOID Argument3;
//     PVOID Argument4;
// } IO_STACK_LOCATION_1_39;
