// Copyright © 2026, Julian Scheffers
// SPDX-License-Identifier: MIT

#![no_std]
#![feature(allocator_api)]

use core::{ffi::c_char, fmt::Write};

pub mod fb;
pub(crate) mod malloc;
#[allow(unused)]
pub(crate) mod sys;

#[repr(transparent)]
pub struct FlantermInner(sys::flanterm_context);

impl FlantermInner {
    /// Write raw bytes to the terminal.
    pub fn write_bytes(&mut self, data: &[u8]) {
        unsafe { sys::flanterm_write(&mut self.0, data.as_ptr() as *mut c_char, data.len()) };
    }

    /// Flush the pending write data.
    pub fn flush(&mut self) {
        unsafe { sys::flanterm_flush(&mut self.0) };
    }

    /// Re-draw the entire terminal.
    pub fn full_refresh(&mut self) {
        unsafe { sys::flanterm_full_refresh(&mut self.0) };
    }

    /// Get the dimensions in terms of rows and columns of characters.
    pub fn get_dimensions(&mut self, cols: &mut usize, rows: &mut usize) {
        unsafe { sys::flanterm_get_dimensions(&mut self.0, cols, rows) };
    }

    /// Whether to flush automatically after writing to the terminal or changing the cursor.
    pub fn set_autoflush(&mut self, autoflush: bool) {
        unsafe { sys::flanterm_set_autoflush(&mut self.0, autoflush) };
    }

    /// Get zero-indexed cursor position.
    pub fn get_cursor_pos(&mut self, x: &mut usize, y: &mut usize) {
        unsafe { sys::flanterm_get_cursor_pos(&mut self.0, x, y) };
    }

    /// Set text foreground colour.
    ///
    /// The colour index uses the terminal's base 0-7 colour range.
    /// Brightness is controlled separately by the bright flag.
    pub fn set_text_fg(&mut self, colour: usize, bright: bool) {
        unsafe { sys::flanterm_set_text_fg(&mut self.0, colour, bright) };
    }

    /// Set text background colour.
    ///
    /// The colour index uses the terminal's base 0-7 colour range.
    /// Brightness is controlled separately by the bright flag.
    pub fn set_text_bg(&mut self, colour: usize, bright: bool) {
        unsafe { sys::flanterm_set_text_bg(&mut self.0, colour, bright) };
    }

    /// Set the text foreground colour to the default.
    pub fn reset_text_fg(&mut self) {
        unsafe { sys::flanterm_reset_text_fg(&mut self.0) };
    }

    /// Set the text background colour to the default.
    pub fn reset_text_bg(&mut self) {
        unsafe { sys::flanterm_reset_text_bg(&mut self.0) };
    }

    /// Clear the terminal.
    ///
    /// If `move_` is `true`, reset cursor position to (0, 0).
    pub fn clear(&mut self, move_: bool) {
        unsafe { sys::flanterm_clear(&mut self.0, move_) };
    }
}

impl Write for FlantermInner {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
