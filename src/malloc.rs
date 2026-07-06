// Copyright © 2026, Julian Scheffers
// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[cfg(feature = "alloc")]
use core::{
    alloc::Layout,
    ptr::{NonNull, null_mut},
};

#[cfg(feature = "alloc")]
use alloc::alloc::{Allocator, Global};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
unsafe extern "C" fn malloc_wrapper(size: usize) -> *mut c_void {
    unsafe {
        match Global.allocate(Layout::from_size_align_unchecked(size, size_of::<usize>())) {
            Ok(mem) => mem.as_ptr() as *mut c_void,
            Err(_) => null_mut(),
        }
    }
}

#[cfg(feature = "alloc")]
unsafe extern "C" fn free_wrapper(ptr: *mut c_void, size: usize) {
    unsafe {
        if let Some(ptr) = NonNull::new(ptr as *mut u8) {
            Global.deallocate(
                ptr,
                Layout::from_size_align_unchecked(size, size_of::<usize>()),
            );
        }
    }
}

#[cfg(feature = "alloc")]
pub const MALLOC: Option<unsafe extern "C" fn(usize) -> *mut c_void> = Some(malloc_wrapper as _);
#[cfg(feature = "alloc")]
pub const FREE: Option<unsafe extern "C" fn(*mut c_void, usize)> = Some(free_wrapper as _);

#[cfg(not(feature = "alloc"))]
pub const MALLOC: Option<unsafe extern "C" fn(usize) -> *mut c_void> = None;
#[cfg(not(feature = "alloc"))]
pub const FREE: Option<unsafe extern "C" fn(*mut c_void, usize)> = None;
