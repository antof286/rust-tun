#![allow(non_snake_case, non_camel_case_types)]
#![cfg(target_os = "windows")]

use windows_sys::core::GUID;
use windows_sys::core::PCWSTR as LPCWSTR;
use windows_sys::Win32::Foundation::BOOL;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::NetworkManagement::Ndis::NET_LUID_LH as NET_LUID;
pub type DWORD = core::ffi::c_ulong;
pub type BYTE = core::ffi::c_uchar;
pub type DWORD64 = core::ffi::c_ulonglong;

/* automatically generated by rust-bindgen 0.66.1 */

pub const WINTUN_MIN_RING_CAPACITY: u32 = 131072;
pub const WINTUN_MAX_RING_CAPACITY: u32 = 67108864;
pub const WINTUN_MAX_IP_PACKET_SIZE: u32 = 65535;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _WINTUN_ADAPTER {
    _unused: [u8; 0],
}
#[doc = " A handle representing Wintun adapter"]
pub type WINTUN_ADAPTER_HANDLE = *mut _WINTUN_ADAPTER;
#[doc = "< Informational"]
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_INFO: WINTUN_LOGGER_LEVEL = 0;
#[doc = "< Warning"]
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_WARN: WINTUN_LOGGER_LEVEL = 1;
#[doc = "< Error"]
pub const WINTUN_LOGGER_LEVEL_WINTUN_LOG_ERR: WINTUN_LOGGER_LEVEL = 2;
#[doc = " Determines the level of logging, passed to WINTUN_LOGGER_CALLBACK."]
pub type WINTUN_LOGGER_LEVEL = ::std::os::raw::c_int;
#[doc = " Called by internal logger to report diagnostic messages\n\n @param Level         Message level.\n\n @param Timestamp     Message timestamp in in 100ns intervals since 1601-01-01 UTC.\n\n @param Message       Message text."]
pub type WINTUN_LOGGER_CALLBACK =
    ::std::option::Option<unsafe extern "stdcall" fn(Level: WINTUN_LOGGER_LEVEL, Timestamp: DWORD64, Message: LPCWSTR)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TUN_SESSION {
    _unused: [u8; 0],
}
#[doc = " A handle representing Wintun session"]
pub type WINTUN_SESSION_HANDLE = *mut _TUN_SESSION;
extern crate libloading;
pub struct wintun {
    __library: ::libloading::Library,
    pub WintunCreateAdapter:
        unsafe extern "stdcall" fn(arg1: LPCWSTR, arg2: LPCWSTR, arg3: *const GUID) -> WINTUN_ADAPTER_HANDLE,
    pub WintunCloseAdapter: unsafe extern "stdcall" fn(arg1: WINTUN_ADAPTER_HANDLE),
    pub WintunOpenAdapter: unsafe extern "stdcall" fn(arg1: LPCWSTR) -> WINTUN_ADAPTER_HANDLE,
    pub WintunGetAdapterLUID: unsafe extern "stdcall" fn(arg1: WINTUN_ADAPTER_HANDLE, arg2: *mut NET_LUID),
    pub WintunGetRunningDriverVersion: unsafe extern "stdcall" fn() -> DWORD,
    pub WintunDeleteDriver: unsafe extern "stdcall" fn() -> BOOL,
    pub WintunSetLogger: unsafe extern "stdcall" fn(arg1: WINTUN_LOGGER_CALLBACK),
    pub WintunStartSession:
        unsafe extern "stdcall" fn(arg1: WINTUN_ADAPTER_HANDLE, arg2: DWORD) -> WINTUN_SESSION_HANDLE,
    pub WintunEndSession: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE),
    pub WintunGetReadWaitEvent: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE) -> HANDLE,
    pub WintunReceivePacket: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE, arg2: *mut DWORD) -> *mut BYTE,
    pub WintunReleaseReceivePacket: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE, arg2: *const BYTE),
    pub WintunAllocateSendPacket: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE, arg2: DWORD) -> *mut BYTE,
    pub WintunSendPacket: unsafe extern "stdcall" fn(arg1: WINTUN_SESSION_HANDLE, arg2: *const BYTE),
}
impl wintun {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let WintunCreateAdapter = __library.get(b"WintunCreateAdapter\0").map(|sym| *sym)?;
        let WintunCloseAdapter = __library.get(b"WintunCloseAdapter\0").map(|sym| *sym)?;
        let WintunOpenAdapter = __library.get(b"WintunOpenAdapter\0").map(|sym| *sym)?;
        let WintunGetAdapterLUID = __library.get(b"WintunGetAdapterLUID\0").map(|sym| *sym)?;
        let WintunGetRunningDriverVersion = __library.get(b"WintunGetRunningDriverVersion\0").map(|sym| *sym)?;
        let WintunDeleteDriver = __library.get(b"WintunDeleteDriver\0").map(|sym| *sym)?;
        let WintunSetLogger = __library.get(b"WintunSetLogger\0").map(|sym| *sym)?;
        let WintunStartSession = __library.get(b"WintunStartSession\0").map(|sym| *sym)?;
        let WintunEndSession = __library.get(b"WintunEndSession\0").map(|sym| *sym)?;
        let WintunGetReadWaitEvent = __library.get(b"WintunGetReadWaitEvent\0").map(|sym| *sym)?;
        let WintunReceivePacket = __library.get(b"WintunReceivePacket\0").map(|sym| *sym)?;
        let WintunReleaseReceivePacket = __library.get(b"WintunReleaseReceivePacket\0").map(|sym| *sym)?;
        let WintunAllocateSendPacket = __library.get(b"WintunAllocateSendPacket\0").map(|sym| *sym)?;
        let WintunSendPacket = __library.get(b"WintunSendPacket\0").map(|sym| *sym)?;
        Ok(wintun {
            __library,
            WintunCreateAdapter,
            WintunCloseAdapter,
            WintunOpenAdapter,
            WintunGetAdapterLUID,
            WintunGetRunningDriverVersion,
            WintunDeleteDriver,
            WintunSetLogger,
            WintunStartSession,
            WintunEndSession,
            WintunGetReadWaitEvent,
            WintunReceivePacket,
            WintunReleaseReceivePacket,
            WintunAllocateSendPacket,
            WintunSendPacket,
        })
    }
    pub unsafe fn WintunCreateAdapter(&self, arg1: LPCWSTR, arg2: LPCWSTR, arg3: *const GUID) -> WINTUN_ADAPTER_HANDLE {
        (self.WintunCreateAdapter)(arg1, arg2, arg3)
    }
    pub unsafe fn WintunCloseAdapter(&self, arg1: WINTUN_ADAPTER_HANDLE) {
        (self.WintunCloseAdapter)(arg1)
    }
    pub unsafe fn WintunOpenAdapter(&self, arg1: LPCWSTR) -> WINTUN_ADAPTER_HANDLE {
        (self.WintunOpenAdapter)(arg1)
    }
    pub unsafe fn WintunGetAdapterLUID(&self, arg1: WINTUN_ADAPTER_HANDLE, arg2: *mut NET_LUID) {
        (self.WintunGetAdapterLUID)(arg1, arg2)
    }
    pub unsafe fn WintunGetRunningDriverVersion(&self) -> DWORD {
        (self.WintunGetRunningDriverVersion)()
    }
    pub unsafe fn WintunDeleteDriver(&self) -> BOOL {
        (self.WintunDeleteDriver)()
    }
    pub unsafe fn WintunSetLogger(&self, arg1: WINTUN_LOGGER_CALLBACK) {
        (self.WintunSetLogger)(arg1)
    }
    pub unsafe fn WintunStartSession(&self, arg1: WINTUN_ADAPTER_HANDLE, arg2: DWORD) -> WINTUN_SESSION_HANDLE {
        (self.WintunStartSession)(arg1, arg2)
    }
    pub unsafe fn WintunEndSession(&self, arg1: WINTUN_SESSION_HANDLE) {
        (self.WintunEndSession)(arg1)
    }
    pub unsafe fn WintunGetReadWaitEvent(&self, arg1: WINTUN_SESSION_HANDLE) -> HANDLE {
        (self.WintunGetReadWaitEvent)(arg1)
    }
    pub unsafe fn WintunReceivePacket(&self, arg1: WINTUN_SESSION_HANDLE, arg2: *mut DWORD) -> *mut BYTE {
        (self.WintunReceivePacket)(arg1, arg2)
    }
    pub unsafe fn WintunReleaseReceivePacket(&self, arg1: WINTUN_SESSION_HANDLE, arg2: *const BYTE) {
        (self.WintunReleaseReceivePacket)(arg1, arg2)
    }
    pub unsafe fn WintunAllocateSendPacket(&self, arg1: WINTUN_SESSION_HANDLE, arg2: DWORD) -> *mut BYTE {
        (self.WintunAllocateSendPacket)(arg1, arg2)
    }
    pub unsafe fn WintunSendPacket(&self, arg1: WINTUN_SESSION_HANDLE, arg2: *const BYTE) {
        (self.WintunSendPacket)(arg1, arg2)
    }
}
