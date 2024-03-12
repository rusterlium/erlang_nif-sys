
use erlang_nif_sys::*;

use std::{mem, ptr};
use std::cmp::min;
use std::sync::atomic::{AtomicIsize, Ordering};

static mut RUSTMAP_TYPE: *const ErlNifResourceType = 0 as *const ErlNifResourceType;
static mut DTOR_COUNTER: Option<AtomicIsize> = None;

nif_init!("mynifmod", [
    ("times2", 1, slice_args!(times2)),
    ("test_enif_make_pid", 0, test_enif_make_pid),
    ("rustmap", 0, rustmap),
    ("rustmap_dtor_count", 0, rustmap_dtor_count),
    ("to_str", 1, slice_args!(to_str)),
    ("hash", 1, slice_args!(hash)),
    ("make_map", 0, slice_args!(make_map)),
    ],
    {load: mynifmod_load});

unsafe fn mynifmod_load(env: *mut ErlNifEnv, _priv_data: *mut *mut c_void, _load_info: ERL_NIF_TERM) -> c_int {
    let mut tried: ErlNifResourceFlags = mem::uninitialized();
    DTOR_COUNTER = Some(AtomicIsize::new(0));
    RUSTMAP_TYPE = enif_open_resource_type(
        env,
        ptr::null(),
        b"rustmap\0".as_ptr(),
        Some(rustmap_destructor),
        ErlNifResourceFlags::ERL_NIF_RT_CREATE,
        &mut tried);
    RUSTMAP_TYPE.is_null() as c_int
}

fn times2(env: *mut ErlNifEnv, args: &[ERL_NIF_TERM]) -> ERL_NIF_TERM {
    unsafe {
        let mut result: i32 = mem::uninitialized();
        if 1==args.len() && 0!=enif_get_int(env, args[0], &mut result) {
            enif_make_int(env, 2*result)
        }
        else {
            enif_make_badarg(env)
        }
    }
}

fn test_enif_make_pid(env: *mut ErlNifEnv, _: c_int, _: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    let mut pid: ErlNifPid = unsafe { mem::uninitialized() };
    unsafe { enif_self(env, &mut pid) };
    unsafe { enif_make_pid(env, &pid) }
}

use std::collections::HashMap;
type RustMap = HashMap<String, String>;


unsafe extern "C" fn rustmap_destructor(_env: *mut ErlNifEnv, handle: *mut c_void) {
    DTOR_COUNTER.as_mut().unwrap().fetch_add(1, Ordering::SeqCst);
    ptr::read(handle as *mut RustMap);
}

unsafe fn rustmap(env: *mut ErlNifEnv, _: c_int, _: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    // Create a value with nontrivial destructor cleanup.
    let mut map = RustMap::new();
    map.insert("Rust".to_string(), "Erlang".to_string());
    map.insert("Erlang".to_string(), "Rust".to_string());

    let mem = enif_alloc_resource(RUSTMAP_TYPE, mem::size_of::<RustMap>());
    assert_eq!(mem as usize % mem::align_of::<RustMap>(), 0);
    ptr::write(mem as *mut RustMap, map);
    let term = enif_make_resource(env, mem);
    enif_release_resource(mem);
    term
}

unsafe fn rustmap_dtor_count(env: *mut ErlNifEnv, _: c_int, _: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    let cnt = DTOR_COUNTER.as_mut().unwrap().load(Ordering::SeqCst);
    enif_make_int(env, cnt as i32)
}

unsafe fn to_str(env: *mut ErlNifEnv, args: &[ERL_NIF_TERM]) -> ERL_NIF_TERM {
    let mut buf = Vec::<u8>::with_capacity(1024);
    let n = enif_snprintf!(buf.as_mut_ptr() as *mut i8,
                           buf.capacity(),
                           "%T".as_ptr() as *mut i8,
                           args[0]);
    if n < 0 {
    enif_make_badarg(env)
    } else {
        let len = min(n as usize, buf.capacity() - 1);
        buf.set_len(len);
        enif_make_string_len(env, buf.as_ptr(), len,
                             ErlNifCharEncoding::ERL_NIF_LATIN1)
    }
}

unsafe fn hash(env: *mut ErlNifEnv, args: &[ERL_NIF_TERM]) -> ERL_NIF_TERM {
    if 1==args.len() {
        let res = enif_hash(ErlNifHash::ERL_NIF_INTERNAL_HASH, args[0], 1234);
        enif_make_uint64(env, res)
    }
    else {
        enif_make_badarg(env)
    }
}

unsafe fn make_map(env: *mut ErlNifEnv, args: &[ERL_NIF_TERM]) -> ERL_NIF_TERM {
    if 0==args.len() {
        let keys: Vec<_> = ["one", "two", "three"].iter()
            .map(|x| enif_make_atom_len(env, x.as_ptr(), x.len()))
            .collect();
        let values: Vec<_> = (1..=3)
            .map(|x| enif_make_int(env, x))
            .collect();
        let mut map = mem::uninitialized();
        if 0!=enif_make_map_from_arrays(env, keys.as_ptr(), values.as_ptr(), keys.len(), &mut map) {
            map
        } else {
            enif_make_badarg(env)
        }
    }
    else {
        enif_make_badarg(env)
    }
}
