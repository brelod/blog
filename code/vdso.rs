use core::ffi::CStr;
use core::mem::transmute;

use crate::elf::*;
use crate::types::*;
use crate::error::{Result, result};

pub struct Vdso {
    time: extern "C" fn(*mut time_t) -> time_t,
    getcpu: extern "C" fn(*mut u32, *mut u32) -> isize,
    gettimeofday: extern "C" fn(*mut timeval, *mut timezone) -> isize,
    clock_getres: extern "C" fn(clockid_t, *mut timespec) -> isize,
    clock_gettime: extern "C" fn(clockid_t, *mut timespec) -> isize,
}

impl Vdso {
    pub(crate) unsafe fn from_ptr(p: *const u8) -> Self {
        let header = &*(p as *const Ehdr);

        let section_headers = core::slice::from_raw_parts(
            p.offset(header.e_shoff as isize) as *const Shdr,
            header.e_shnum as usize
        );

        let dynstr = section_headers.iter().find(|e| e.sh_type == 3).map(|h| {
            p.offset(h.sh_offset as isize) as *const u8
        }).unwrap();

        let dynsym = section_headers.iter().find(|e| e.sh_type == 11).map(|h| {
            core::slice::from_raw_parts(
                p.offset(h.sh_offset as isize) as *const Sym,
                h.sh_size as usize / core::mem::size_of::<Sym>(),
            )
        }).unwrap();

        let mut time = None;
        let mut getcpu = None;
        let mut gettimeofday = None;
        let mut clock_getres = None;
        let mut clock_gettime = None;

        for symbole in dynsym {
            let s = dynstr.add(symbole.st_name as usize) as *const i8;
            match CStr::from_ptr(s).to_str() {
                Ok("time") => { time = transmute(p.add(symbole.st_value as usize)); }
                Ok("getcpu") => { getcpu = transmute(p.add(symbole.st_value as usize)); }
                Ok("gettimeofday") => { gettimeofday = transmute(p.add(symbole.st_value as usize)); }
                Ok("clock_getres") => { clock_getres = transmute(p.add(symbole.st_value as usize)); }
                Ok("clock_gettime") => { clock_gettime = transmute(p.add(symbole.st_value as usize)); }
                _ => { /* ignore */ }
            }
        }

        Self {
            time: time.unwrap(),
            getcpu: getcpu.unwrap(),
            gettimeofday: gettimeofday.unwrap(),
            clock_getres: clock_getres.unwrap(),
            clock_gettime: clock_gettime.unwrap(),
        }
    }

    #[inline(always)]
    pub fn time(&self, time: &mut time_t) -> time_t {
        (self.time)(time as *mut _)
    }

    /// The signature of this system call is different from the one documented in the man pages.
    /// This is because there is only way make this system call fail which is providing invalid pointers
    /// Since returning Result<()> has the same effect as returning a tuple with two numbers we simply
    /// make sure that it never fails by putting these variables on the stack.
    ///
    /// TODO: Is this really true about the Result<()>????
    #[inline(always)]
    pub fn getcpu(&self) -> (u32, u32) {
        let mut cpu = 0;
        let mut node = 0;
        (self.getcpu)(&mut cpu as *mut _, &mut node as *mut _);
        (cpu, node)
    }

    #[inline(always)]
    pub fn gettimeofday(&self, tv: &mut timeval, tz: &mut timezone) -> Result<()> {
        result((self.gettimeofday)(tv as *mut _, tz as *mut _)).map(|_| ())
    }

    #[inline(always)]
    pub fn clock_getres(&self, clock: clockid_t, spec: &mut timespec) -> Result<()> {
        result((self.clock_getres)(clock, spec as *mut _)).map(|_| ())
    }

    #[inline(always)]
    pub fn clock_gettime(&self, clock: clockid_t, spec: &mut timespec) -> Result<()> {
        result((self.clock_gettime)(clock, spec as *mut _)).map(|_| ())
    }
}
