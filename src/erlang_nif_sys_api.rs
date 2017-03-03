
#[cfg(windows)]
use unreachable::UncheckedOptionExt;  // unchecked unwrap used in generated Windows code

pub use std::os::raw::{c_int, c_void, c_uint, c_char, c_uchar, c_ulong, c_long, c_double};


#[allow(non_camel_case_types)]
pub type size_t = usize;

use std::option::Option;

#[allow(non_camel_case_types)]
pub type ERL_NIF_UINT = size_t;

#[allow(non_camel_case_types)]
pub type ERL_NIF_TERM = ERL_NIF_UINT;

//#[derive(Debug, Copy, Clone)]
//#[repr(C)]
//pub struct ERL_NIF_TERM(ERL_NIF_UINT);  // Don't do this, 32 bit calling convention is different for structs and ints.


/// See [ErlNifEnv](http://www.erlang.org/doc/man/erl_nif.html#ErlNifEnv) in the Erlang docs.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct ErlNifEnv {
    dummy: *mut c_void,  // block automatic Send and Sync traits.  Ref https://doc.rust-lang.org/beta/nomicon/send-and-sync.html
}

// Ownership of an env may be safely transfers between threads, therefore ErlNifEnv is Send.
// This is the common use case for process independent environments created with enif_alloc_env().
// ErlNifEnv is NOT Sync because it is thread unsafe.
unsafe impl Send for ErlNifEnv {}


/// See [ErlNifFunc](http://www.erlang.org/doc/man/erl_nif.html#ErlNifFunc) in the Erlang docs.
// #[allow(missing_copy_implementations)]
#[repr(C)]
pub struct ErlNifFunc {
    pub name:     *const u8,
    pub arity:    c_uint,
    pub function: unsafe extern "C" fn(env: *mut ErlNifEnv, argc: c_int, argv: *const ERL_NIF_TERM) -> ERL_NIF_TERM,
    pub flags:    c_uint,
}

// #[allow(missing_copy_implementations)]
#[doc(hidden)]
#[derive(Debug)]
#[repr(C)]
pub struct ErlNifEntry {
    pub major:        c_int,
    pub minor:        c_int,
    pub name:         *const u8,
    pub num_of_funcs: c_int,
    pub funcs:        *const ErlNifFunc,
    pub load:    Option<unsafe extern "C" fn(env: *mut ErlNifEnv, priv_data: *mut *mut c_void, load_info: ERL_NIF_TERM)-> c_int>,
    pub reload:  Option<unsafe extern "C" fn(env: *mut ErlNifEnv, priv_data: *mut *mut c_void, load_info: ERL_NIF_TERM) -> c_int>,
    pub upgrade: Option<unsafe extern "C" fn(env: *mut ErlNifEnv, priv_data: *mut *mut c_void, old_priv_data: *mut *mut c_void, load_info: ERL_NIF_TERM) -> c_int>,
    pub unload:  Option<unsafe extern "C" fn(env: *mut ErlNifEnv, priv_data: *mut c_void) -> ()>,
    pub vm_variant: *const u8,
    pub options: c_uint,
}

pub const ERL_NIF_DIRTY_NIF_OPTION: c_uint = 1;

/// See [ErlNifBinary](http://www.erlang.org/doc/man/erl_nif.html#ErlNifBinary) in the Erlang docs.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
pub struct ErlNifBinary {
    pub size: size_t,
    pub data: *mut u8,
    bin_term: ERL_NIF_TERM,
    ref_bin: *mut c_void,
}

/// See [ErlNifResourceType](http://www.erlang.org/doc/man/erl_nif.html#ErlNifResourceType) in the Erlang docs.
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct ErlNifResourceType {dummy:c_int}

/// See [ErlNifResourceDtor](http://www.erlang.org/doc/man/erl_nif.html#ErlNifResourceDtor) in the Erlang docs.
#[allow(missing_copy_implementations)]
pub type ErlNifResourceDtor = unsafe extern "C" fn(arg1: *mut ErlNifEnv, arg2: *mut c_void) -> ();

/// See [ErlNifResourceFlags](http://www.erlang.org/doc/man/erl_nif.html#ErlNifResourceFlags) in the Erlang docs.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum ErlNifResourceFlags {
    ERL_NIF_RT_CREATE = 1,
    ERL_NIF_RT_TAKEOVER = 2,
}

/// See [ErlNifCharEncoding](http://www.erlang.org/doc/man/erl_nif.html#ErlNifCharEncoding) in the Erlang docs.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum ErlNifCharEncoding {
    ERL_NIF_LATIN1 = 1,
    DUMMY = 999, // prevents "univariant enum" compile error
}

/// See [ErlNifPid](http://www.erlang.org/doc/man/erl_nif.html#ErlNifPid) in the Erlang docs.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ErlNifPid {
    pid: ERL_NIF_TERM,
}

/// See [enif_make_pid](http://erlang.org/doc/man/erl_nif.html#enif_make_pid) in the Erlang docs
pub unsafe fn enif_make_pid(_env: *mut ErlNifEnv, pid: & ErlNifPid) -> ERL_NIF_TERM {
    pid.pid
}

/// See [ErlNifSysInfo](http://www.erlang.org/doc/man/erl_nif.html#ErlNifSysInfo) in the Erlang docs.
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct ErlNifSysInfo {
    pub driver_major_version: c_int,
    pub driver_minor_version: c_int,
    pub erts_version: *mut c_char,
    pub otp_release: *mut c_char,
    pub thread_support: c_int,
    pub smp_support: c_int,
    pub async_threads: c_int,
    pub scheduler_threads: c_int,
    pub nif_major_version: c_int,
    pub nif_minor_version: c_int,
    pub dirty_scheduler_support: c_int,
}

// /// See [ErlNifFunc](http://www.erlang.org/doc/man/erl_nif.html#ErlNifFunc) in the Erlang docs.
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub enum ErlNifDirtyTaskFlags {
//     ERL_NIF_DIRTY_JOB_CPU_BOUND = 1,
//     ERL_NIF_DIRTY_JOB_IO_BOUND = 2,
// }

pub type ErlNifDirtyTaskFlags = c_uint;
pub const ERL_NIF_DIRTY_JOB_CPU_BOUND: ErlNifDirtyTaskFlags = 1;
pub const ERL_NIF_DIRTY_JOB_IO_BOUND: ErlNifDirtyTaskFlags = 2;


/// See [ErlNifMapIterator](http://www.erlang.org/doc/man/erl_nif.html#ErlNifMapIterator) in the Erlang docs.
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct ErlNifMapIterator {
    map: ERL_NIF_TERM,
    t_limit: ERL_NIF_UINT,
    idx: ERL_NIF_UINT,
    ks: *mut ERL_NIF_TERM,
    vs: *mut ERL_NIF_TERM,
    __spare__: [*mut c_void; 2],
}

/// See [ErlNifMapIteratorEntry](http://www.erlang.org/doc/man/erl_nif.html#ErlNifMapIteratorEntry) in the Erlang docs.
#[derive(Copy, Clone)]
#[repr(C)]
pub enum ErlNifMapIteratorEntry {
    ERL_NIF_MAP_ITERATOR_HEAD = 1,
    ERL_NIF_MAP_ITERATOR_TAIL = 2,
}

/// See [ErlNifTime](http://www.erlang.org/doc/man/erl_nif.html#ErlNifTime) in the Erlang docs.
pub type ErlNifTime = i64;

/// Error return value for `enif_monotonic_time()`, `enif_time_offset()`, and `enif_convert_time_unit()`.
pub const ERL_NIF_TIME_ERROR: i64 = -9223372036854775808;
//const ERL_NIF_TIME_ERROR:i64 = i64::min_value();  "error: const fn's not yet stable"

/// See [ErlNifTimeUnit](http://www.erlang.org/doc/man/erl_nif.html#ErlNifTimeUnit) in the Erlang docs.
#[derive(Copy, Clone)]
#[repr(C)]
pub enum ErlNifTimeUnit {
    // values yanked from https://github.com/erlang/otp/blob/7cb403e4aa044fd2cc7702dbe8e2d0eea68e81f3/erts/emulator/beam/erl_drv_nif.h#L132
    ERL_NIF_SEC  = 0,
    ERL_NIF_MSEC = 1,
    ERL_NIF_USEC = 2,
    ERL_NIF_NSEC = 3,
}

/// See [ErlNifUniqueInteger](http://erlang.org/doc/man/erl_nif.html#ErlNifUniqueInteger) in the Erlang docs.
pub type ErlNifUniqueInteger = c_int;
pub const ERL_NIF_UNIQUE_POSITIVE: ErlNifUniqueInteger  = (1 << 0);
pub const ERL_NIF_UNIQUE_MONOTONIC: ErlNifUniqueInteger = (1 << 1);
// ref https://github.com/erlang/otp/blob/maint/erts/emulator/beam/erl_nif.h#L203
// FIXME: Should actually be C enum, but repr(C) enums in Rust can't be used as bitfields.
//        Fix if the right abstraction ever lands in Rust.


/// See [ErlNifPort](http://erlang.org/doc/man/erl_nif.html#ErlNifPort) in the Erlang docs.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErlNifPort {
    port_id: ERL_NIF_TERM,  // internal, may change
}
// ref https://github.com/erlang/otp/blob/maint/erts/emulator/beam/erl_nif.h#L155


/// See [ErlNifBinaryToTerm](http://erlang.org/doc/man/erl_nif.html#ErlNifBinaryToTerm) in the Erlang docs.
pub type ErlNifBinaryToTerm = c_int;
pub const ERL_NIF_BIN2TERM_SAFE: ErlNifBinaryToTerm = 0x20000000;


pub const ERL_NIF_THR_UNDEFINED: c_int =  0;
pub const ERL_NIF_THR_NORMAL_SCHEDULER: c_int =  1;
pub const ERL_NIF_THR_DIRTY_CPU_SCHEDULER: c_int =  2;
pub const ERL_NIF_THR_DIRTY_IO_SCHEDULER: c_int =  3;



include!(concat!(env!("OUT_DIR"), "/nif_api.snippet"));
// example of included content:
// extern "C" {
//     pub fn enif_priv_data(arg1: *mut ErlNifEnv) -> *mut c_void;
//     pub fn enif_alloc(size: size_t) -> *mut c_void;
//     pub fn enif_free(ptr: *mut c_void);
//     pub fn enif_is_atom(arg1: *mut ErlNifEnv, term: ERL_NIF_TERM) -> c_int;
//     pub fn enif_is_binary(arg1: *mut ErlNifEnv, term: ERL_NIF_TERM) -> c_int;
// ...

