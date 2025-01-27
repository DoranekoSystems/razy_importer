#![cfg(test)]

extern crate speculate;

use std::ffi::CString;

use speculate::speculate;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryA};

use crate::OffsetHashPair;

speculate! {
    describe "razy-importer" {
        fn gen_ohp(value: &str) -> OffsetHashPair {
            let random_u32: u32 = const_random!(u32);
            let mut random_array: [u8; 128] = const_random!([u8; 128]);
            if let Some(last) = random_array.last_mut() {
                *last = 0;
            }
            let hash: OffsetHashPair = crate::hash::khash(
                value.as_bytes(),
                crate::hash::khash_impl(&random_array, random_u32, false),
                false,
            );
            return hash;
        }

        fn check_ex(dll_name: &str, func_name: &str) -> (u64, u64) {
            unsafe {
                let dll_name_c: CString = CString::new(dll_name).unwrap();
                let dll_base: u64 = GetModuleHandleA(dll_name_c.as_ptr()) as _;
                let func_name_c: CString = CString::new(func_name).unwrap();
                let func_addr: u64 = GetProcAddress(dll_base as _, func_name_c.as_ptr()) as _;
                return (dll_base, func_addr);
            }
        }

        fn check(dll_name: &str, func_name: &str) {
            unsafe {
                let (dll_base, func_addr) = check_ex(dll_name, func_name);
                let base = crate::get_module_base(gen_ohp(dll_name), false);
                assert_eq_hex!(base, dll_base);
                let addr = crate::get_export(base, gen_ohp(func_name), false);
                assert_eq_hex!(addr, func_addr);
                let addr = crate::get_export_forwarded(gen_ohp(func_name), false);
                assert_eq_hex!(addr, func_addr);
            };
        }

        fn check_forward(func_name: &str) {
            unsafe {
                let addr = crate::get_export_forwarded(gen_ohp(func_name), false);
                assert_ne_hex!(addr, 0);
            }
        }

        #[allow(dead_code)]
        fn load_lib(dll_name: &str) {
            let dll_name_c: CString = CString::new(dll_name).unwrap();
            unsafe { LoadLibraryA(dll_name_c.as_ptr()) };
        }

        it "check ntdll" {
            // non-forwarded
            check("ntdll.dll", "NtGetCurrentProcessorNumber");
            check("ntdll.dll", "NtGetCurrentProcessorNumberEx");
            check("ntdll.dll", "ZwGetCurrentProcessorNumber");
            check("ntdll.dll", "ZwGetCurrentProcessorNumberEx");
            check("ntdll.dll", "NtOpenProcess");
            check("ntdll.dll", "NtOpenProcessToken");
            check("ntdll.dll", "NtOpenProcessTokenEx");
            check("ntdll.dll", "ZwOpenProcess");
            check("ntdll.dll", "ZwOpenProcessToken");
            check("ntdll.dll", "ZwOpenProcessTokenEx");
            check("ntdll.dll", "NtOpenThread");
            check("ntdll.dll", "NtOpenThreadToken");
            check("ntdll.dll", "NtOpenThreadTokenEx");
            check("ntdll.dll", "ZwOpenThread");
            check("ntdll.dll", "ZwOpenThreadToken");
            check("ntdll.dll", "ZwOpenThreadTokenEx");
        }

        it "check_forward ntdll" {
            check_forward("NtGetCurrentProcessorNumber");
            check_forward("NtGetCurrentProcessorNumberEx");
            check_forward("ZwGetCurrentProcessorNumber");
            check_forward("ZwGetCurrentProcessorNumberEx");
            check_forward("NtOpenProcess");
            check_forward("NtOpenProcessToken");
            check_forward("NtOpenProcessTokenEx");
            check_forward("ZwOpenProcess");
            check_forward("ZwOpenProcessToken");
            check_forward("ZwOpenProcessTokenEx");
            check_forward("NtOpenThread");
            check_forward("NtOpenThreadToken");
            check_forward("NtOpenThreadTokenEx");
            check_forward("ZwOpenThread");
            check_forward("ZwOpenThreadToken");
            check_forward("ZwOpenThreadTokenEx");
        }

        it "check kernel32" {
            // includes forwarded hashes
            check("kernel32.dll", "OpenProcess");
            check("kernel32.dll", "OpenThread");
            check("kernel32.dll", "GetCurrentProcess");
            check("kernel32.dll", "GetCurrentProcessId");
            check("kernel32.dll", "GetCurrentThread");
            check("kernel32.dll", "GetCurrentThreadId");
            check("kernel32.dll", "GetCurrentUmsThread");
            check("kernel32.dll", "GetCurrentActCtx");
            check("kernel32.dll", "GetCurrentActCtxWorker");
            check("kernel32.dll", "GetCurrentApplicationUserModelId");
        }

        it "check_forward kernel32" {
            check_forward("OpenProcess");
            check_forward("OpenThread");
            check_forward("GetCurrentProcess");
            check_forward("GetCurrentProcessId");
            check_forward("GetCurrentThread");
            check_forward("GetCurrentThreadId");
            check_forward("GetCurrentUmsThread");
            check_forward("GetCurrentActCtx");
            check_forward("GetCurrentActCtxWorker");
            check_forward("GetCurrentApplicationUserModelId");
        }

        it "check should not throw" {
            unsafe {
                let dll_name: &str = "ntdll.dll";
                let base = crate::get_module_base(gen_ohp(dll_name), false);
                let func_name: &str = "NonExistentAPINameShouldNotThrowException";
                let addr = crate::get_export(base, gen_ohp(func_name), false);
                assert_eq_hex!(addr, 0);
                // This throws exception, as intended.
                // let addr = crate::get_export(0, gen_ohp(func_name), false);
                // assert_eq_hex!(addr, 0);
            }
        }

        it "check_forward should not throw" {
            unsafe {
                let func_name: &str = "NonExistentAPINameShouldNotThrowException";
                let addr = crate::get_export_forwarded(gen_ohp(func_name), false);
                assert_eq_hex!(addr, 0);
            }
        }
    }
}
