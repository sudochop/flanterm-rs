// Copyright © 2026, Julian Scheffers
// SPDX-License-Identifier: MIT

use core::{
    ffi::{c_int, c_void},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::{NonNull, null},
};

use crate::{FlantermInner, malloc, sys};

#[repr(transparent)]
pub struct FlantermFbInner(pub FlantermInner);

pub struct FlantermFb<'a> {
    ptr: NonNull<FlantermFbInner>,
    marker: PhantomData<&'a mut [u32]>,
}

pub trait FbFlushCallback {
    fn on_flush(framebuffer: &[u8]);
}

impl<'a> FlantermFb<'a> {
    pub fn new(
        framebuffer: &'a mut [u32],
        width: usize,
        height: usize,
        pitch: usize,
        red_mask_size: u8,
        red_mask_shift: u8,
        green_mask_size: u8,
        green_mask_shift: u8,
        blue_mask_size: u8,
        blue_mask_shift: u8,
        font: Option<Font<'a>>,
        font_scale_x: usize,
        font_scale_y: usize,
        background: Option<&'a [u32]>,
        ansi_colours: Option<&'a [u32; 8]>,
        ansi_bright_colours: Option<&'a [u32; 8]>,
        default_bg: Option<u32>,
        default_bg_bright: Option<u32>,
        default_fg: Option<u32>,
        default_fg_bright: Option<u32>,
        margin: usize,
        rotation: Rotation,
    ) -> Option<Self> {
        unsafe {
            assert!(
                framebuffer.len() >= height * pitch,
                "FlantermFb::new(): framebuffer is not large enough given its height and pitch"
            );
            if let Some(background) = background {
                assert!(
                    background.len() >= height * width,
                    "FlantermFb::new(): background is not large enough given its height and width"
                );
            }
            let ctx = sys::flanterm_fb_init(
                malloc::MALLOC,
                malloc::FREE,
                framebuffer.as_mut_ptr(),
                width,
                height,
                pitch,
                red_mask_size,
                red_mask_shift,
                green_mask_size,
                green_mask_shift,
                blue_mask_size,
                blue_mask_shift,
                background.map_or(null(), |x| x.as_ptr()) as *mut u32,
                ansi_colours.map_or(null(), |x| x.as_ptr()) as *mut u32,
                ansi_bright_colours.map_or(null(), |x| x.as_ptr()) as *mut u32,
                default_bg
                    .as_ref()
                    .map(|x| x as *const u32)
                    .unwrap_or(null()) as *mut u32,
                default_fg
                    .as_ref()
                    .map(|x| x as *const u32)
                    .unwrap_or(null()) as *mut u32,
                default_bg_bright
                    .as_ref()
                    .map(|x| x as *const u32)
                    .unwrap_or(null()) as *mut u32,
                default_fg_bright
                    .as_ref()
                    .map(|x| x as *const u32)
                    .unwrap_or(null()) as *mut u32,
                font.map_or(null(), |x| x.font.as_ptr()) as *mut c_void,
                font.map_or(0, |x| x.width),
                font.map_or(0, |x| x.height),
                font.map_or(0, |x| x.spacing),
                font_scale_x,
                font_scale_y,
                margin,
                rotation as c_int,
            );
            Some(Self {
                ptr: NonNull::new(ctx as *mut FlantermFbInner)?,
                marker: PhantomData,
            })
        }
    }

    pub fn set_flush_callback_raw(
        &mut self,
        callback: Option<unsafe extern "C" fn(*mut c_void, usize)>,
    ) {
        unsafe {
            sys::flanterm_fb_set_flush_callback(&raw mut self.ptr.as_mut().0.0, callback);
        }
    }

    pub fn set_flush_callback<T: FbFlushCallback>(&mut self) {
        unsafe extern "C" fn wrapper<T: FbFlushCallback>(addr: *mut c_void, size: usize) {
            T::on_flush(unsafe { &*core::ptr::slice_from_raw_parts(addr as *const u8, size) });
        }
        self.set_flush_callback_raw(Some(wrapper::<T> as _));
    }

    pub fn remove_flush_callback(&mut self) {
        unsafe {
            sys::flanterm_fb_set_flush_callback(&raw mut self.ptr.as_mut().0.0, None);
        }
    }
}

impl Drop for FlantermFb<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::flanterm_deinit(&raw mut self.ptr.as_mut().0.0, malloc::FREE);
        }
    }
}

impl Deref for FlantermFb<'_> {
    type Target = FlantermFbInner;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for FlantermFb<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Deref for FlantermFbInner {
    type Target = FlantermInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlantermFbInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

/// Flanterm font definition.
#[derive(Clone, Copy)]
pub struct Font<'a> {
    pub font: &'a [u8],
    pub width: usize,
    pub height: usize,
    pub spacing: usize,
}
