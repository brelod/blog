use core::ffi::CStr;
use core::sync::atomic::Ordering;

use crate::types::{auxv_t, AT};

pub struct Pointers {
    next: isize,
    ptrs: *const *const i8,
}

impl core::iter::Iterator for Pointers {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let ptr = *self.ptrs.offset(self.next);
            self.next += 1;
            match ptr.is_null() {
                true => None,
                false => CStr::from_ptr(ptr).to_str().ok(),
            }
        }
    }
}

pub struct Variables {
    ptrs: Pointers,
}

impl core::iter::Iterator for Variables {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptrs.next().map(|s| s.split_once('=')).flatten()
    }
}

pub fn args() -> Pointers {
    Pointers {
        next: 0,
        ptrs: crate::ARGV.load(Ordering::Relaxed),
    }
}

pub fn envp() -> Pointers {
    Pointers {
        next: 0,
        ptrs: crate::ENVP.load(Ordering::Relaxed),
    }
}

pub fn vars() -> Variables {
    Variables { ptrs: envp() }
}

pub fn var(key: &str) -> Option<&'static str> {
    vars().find(|(k, _)| *k == key).map(|(_, v)| v)
}

// ==============================================================================
// Aux Vector
// ==============================================================================
/* ANCHOR: auxv */
pub struct AuxVector {
    next: isize,
    buf: *const auxv_t,
}

impl core::iter::Iterator for AuxVector {
    type Item = AT;

    fn next(&mut self) -> Option<Self::Item> {
        let aux = unsafe { *self.buf.offset(self.next) };
        self.next += 1;

        match AT::from(aux){
            AT::AT_NULL => None,
            other => Some(other),
        }
    }
}

pub fn auxv() -> AuxVector {
    AuxVector { 
        next: 0,
        buf: crate::AUXV.load(Ordering::Relaxed),
    }
}
/* ANCHOR_END: auxv */


